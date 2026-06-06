//! Lightweight HTTP client for the Dracon AI API.
//!
//! Consumers depend on this crate instead of `dracon-ai-client` directly.
//! All AI logic lives in the API server; this crate is just a typed HTTP wrapper.
//!
//! # Quick start
//!
//! ```no_run
//! use dracon_ai_sdk::DraconAi;
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let ai = DraconAi::new("http://localhost:3001", "my-api-key");
//!
//! let response = ai.chat("free", "my-app", vec![
//!     dracon_ai_sdk::ChatMessage::user("Hello!"),
//! ]).await?;
//! println!("{}", response.content);
//! # Ok(()) }
//! ```
//!
//! # Environment variables
//!
//! - `AI_API_URL` — API base URL (default: `http://localhost:3001`)
//! - `AI_API_KEY` — API key for authentication
//!
//! Use [`DraconAi::from_env`] to read both at once.

mod error;
pub mod types;

pub use error::{Result, SdkError};
pub use types::{
    AudioResponse, ChatChunk, ChatMessage, ChatRequest, ChatResponse, HealthResponse,
    ImageRequest, ImageResponse, MusicRequest, Role, SoundFxRequest, TtsRequest, UsageStats,
};

use reqwest::Client as HttpClient;
use std::time::Duration;

use futures_util::stream::{Stream, StreamExt};

/// The main AI client. Holds a base URL, API key, and an HTTP client.
#[derive(Debug, Clone)]
pub struct DraconAi {
    base_url: String,
    api_key: String,
    http: HttpClient,
}

impl DraconAi {
    /// Create a new client with explicit URL and key.
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            api_key: api_key.into(),
            http: HttpClient::builder()
                .timeout(Duration::from_secs(120))
                .build()
                .expect("reqwest client should build"),
        }
    }

    /// Create a client from environment variables.
    ///
    /// Reads:
    /// - `AI_API_URL` (default: `http://localhost:3001`)
    /// - `AI_API_KEY` (required)
    pub fn from_env() -> Result<Self> {
        let api_key = std::env::var("AI_API_KEY")
            .map_err(|_| SdkError::MissingEnv("AI_API_KEY".to_string()))?;
        let base_url = std::env::var("AI_API_URL")
            .unwrap_or_else(|_| "http://localhost:3001".to_string());
        Ok(Self::new(base_url, api_key))
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    /// Send a chat completion request.
    pub async fn chat(
        &self,
        lane: &str,
        project_id: &str,
        messages: Vec<ChatMessage>,
    ) -> Result<ChatResponse> {
        let req = ChatRequest {
            lane: lane.to_string(),
            project_id: project_id.to_string(),
            messages,
            model: None,
            max_tokens: None,
        };
        self.chat_with_options(req).await
    }

    /// Send a chat completion request with full options.
    pub async fn chat_with_options(&self, req: ChatRequest) -> Result<ChatResponse> {
        let url = self.url("/v1/ai/chat/completions");
        let resp = self
            .http
            .post(&url)
            .header("x-api-key", &self.api_key)
            .json(&req)
            .send()
            .await?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(SdkError::from_status(status.as_u16(), &body));
        }
        Ok(resp.json::<ChatResponse>().await?)
    }

    /// Send a chat completion request and stream the response as SSE chunks.
    ///
    /// Returns a `Stream<Item = Result<ChatChunk, SdkError>>` of incremental
    /// tokens from the upstream provider. The stream ends when the server
    /// closes the SSE connection.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dracon_ai_sdk::DraconAi;
    /// use futures_util::StreamExt;
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let ai = DraconAi::new("http://localhost:3001", "my-api-key");
    /// let messages = vec![dracon_ai_sdk::ChatMessage::user("Hi!")];
    /// let mut stream = ai.chat_stream("free", "my-app", messages).await?;
    /// while let Some(chunk) = stream.next().await {
    ///     print!("{}", chunk?.content);
    /// }
    /// # Ok(()) }
    /// ```
    pub async fn chat_stream(
        &self,
        lane: &str,
        project_id: &str,
        messages: Vec<ChatMessage>,
    ) -> Result<impl Stream<Item = Result<ChatChunk>> + Send + 'static> {
        let req = ChatRequest {
            lane: lane.to_string(),
            project_id: project_id.to_string(),
            messages,
            model: None,
            max_tokens: None,
        };
        self.chat_stream_with_options(req).await
    }

    /// Streaming chat with full request options.
    pub async fn chat_stream_with_options(
        &self,
        req: ChatRequest,
    ) -> Result<impl Stream<Item = Result<ChatChunk>> + Send + 'static> {
        let url = self.url("/v1/ai/chat/stream");
        let resp = self
            .http
            .post(&url)
            .header("x-api-key", &self.api_key)
            .json(&req)
            .send()
            .await?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(SdkError::from_status(status.as_u16(), &body));
        }

        // Wrap reqwest's byte stream in our SSE-aware parser. We do all parsing
        // inside `SseChunkStream` so that the public return type is just
        // `Result<ChatChunk>` per item — no `StreamEnd` sentinel leaks through
        // because the parser skips `data: [DONE]` and returns `None` from the
        // stream when the server closes the connection.
        let string_stream = resp
            .bytes_stream()
            .map(|chunk_result| {
                chunk_result.map_err(|e| SdkError::StreamTransport(e.to_string()))
            })
            .map(|res| res.map(|bytes| String::from_utf8_lossy(&bytes).to_string()));

        Ok(SseChunkStream::new(Box::pin(string_stream)))
    }

    /// Generate an image.
    pub async fn image(
        &self,
        lane: &str,
        project_id: &str,
        prompt: &str,
    ) -> Result<ImageResponse> {
        let url = self.url("/v1/ai/image");
        let req = ImageRequest {
            lane: lane.to_string(),
            project_id: project_id.to_string(),
            prompt: prompt.to_string(),
            model: None,
        };
        self.post_json(&url, &req).await
    }

    /// Generate music.
    pub async fn music(
        &self,
        lane: &str,
        project_id: &str,
        prompt: &str,
    ) -> Result<AudioResponse> {
        let url = self.url("/v1/ai/music");
        let req = MusicRequest {
            lane: lane.to_string(),
            project_id: project_id.to_string(),
            prompt: prompt.to_string(),
            model: None,
        };
        self.post_json(&url, &req).await
    }

    /// Generate sound effects.
    pub async fn soundfx(
        &self,
        lane: &str,
        project_id: &str,
        prompt: &str,
    ) -> Result<AudioResponse> {
        let url = self.url("/v1/ai/soundfx");
        let req = SoundFxRequest {
            lane: lane.to_string(),
            project_id: project_id.to_string(),
            prompt: prompt.to_string(),
            model: None,
        };
        self.post_json(&url, &req).await
    }

    /// Synthesize speech from text.
    pub async fn tts(
        &self,
        lane: &str,
        project_id: &str,
        text: &str,
    ) -> Result<AudioResponse> {
        let url = self.url("/v1/ai/tts");
        let req = TtsRequest {
            lane: lane.to_string(),
            project_id: project_id.to_string(),
            text: text.to_string(),
            model: None,
        };
        self.post_json(&url, &req).await
    }

    /// Check API health.
    pub async fn health(&self) -> Result<HealthResponse> {
        let url = self.url("/healthz");
        let resp = self.http.get(&url).send().await?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(SdkError::from_status(status.as_u16(), &body));
        }
        Ok(resp.json::<HealthResponse>().await?)
    }

    /// Internal helper: POST JSON, check status, parse response.
    async fn post_json<B: serde::Serialize, R: serde::de::DeserializeOwned>(
        &self,
        url: &str,
        body: &B,
    ) -> Result<R> {
        let resp = self
            .http
            .post(url)
            .header("x-api-key", &self.api_key)
            .json(body)
            .send()
            .await?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(SdkError::from_status(status.as_u16(), &body));
        }
        Ok(resp.json::<R>().await?)
    }
}
