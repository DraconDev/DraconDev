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

use futures::Stream;
use reqwest::Client as HttpClient;
use std::pin::Pin;

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
                .timeout(std::time::Duration::from_secs(120))
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

    /// Send a streaming chat completion request. Returns a stream of chunks.
    pub async fn chat_stream(
        &self,
        lane: &str,
        project_id: &str,
        messages: Vec<ChatMessage>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<ChatChunk>> + Send>>> {
        let req = ChatRequest {
            lane: lane.to_string(),
            project_id: project_id.to_string(),
            messages,
            model: None,
            max_tokens: None,
        };
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
        // Parse SSE stream
        let stream = resp.bytes_stream();
        let sse_stream = parse_sse_stream(stream);
        Ok(Box::pin(sse_stream))
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
        Ok(resp.json::<ImageResponse>().await?)
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
        Ok(resp.json::<AudioResponse>().await?)
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
        Ok(resp.json::<AudioResponse>().await?)
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
        Ok(resp.json::<AudioResponse>().await?)
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
}

/// Parse a Server-Sent Events byte stream into a stream of ChatChunk results.
fn parse_sse_stream(
    byte_stream: impl Stream<Item = Result<bytes::Bytes, reqwest::Error>> + Send + 'static,
) -> impl Stream<Item = Result<ChatChunk>> + Send {
    use futures::stream::{StreamExt, TryStreamExt};

    // Convert to a stream of Strings by buffering lines
    async_stream::stream! {
        let mut buffer = String::new();
        let mut stream = byte_stream.map_err(|e| SdkError::Transport(e));

        while let Some(chunk_result) = stream.next().await {
            let chunk = match chunk_result {
                Ok(c) => c,
                Err(e) => {
                    yield Err(e);
                    continue;
                }
            };
            buffer.push_str(&String::from_utf8_lossy(&chunk));

            // Process complete SSE events (lines ending with \n\n)
            while let Some(idx) = buffer.find("\n\n") {
                let event: String = buffer.drain(..idx + 2).collect();
                for line in event.lines() {
                    if let Some(data) = line.strip_prefix("data: ") {
                        if data.trim().is_empty() {
                            continue;
                        }
                        match serde_json::from_str::<ChatChunk>(data) {
                            Ok(chunk) => yield Ok(chunk),
                            Err(e) => yield Err(SdkError::Parse(e)),
                        }
                    }
                }
            }
        }
    }
}
