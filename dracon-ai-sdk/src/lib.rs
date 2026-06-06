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

        // Parse SSE: events separated by blank lines, payload in `data: <json>` lines.
        // Skip event-type lines; tolerate comment lines starting with `:`.
        // The server emits each chunk as a complete SSE event; the final frame
        // has no content and is dropped (the stream simply ends when the body ends).
        let byte_stream = resp.bytes_stream();
        let event_stream = byte_stream
            .map(|chunk_result| {
                chunk_result.map_err(|e| SdkError::Http(e.to_string()))
            })
            .map(|chunk_result| match chunk_result {
                Ok(bytes) => Ok(String::from_utf8_lossy(&bytes).to_string()),
                Err(e) => Err(e),
            })
            .scan(String::new(), |buffer, line_result: Result<String>| {
                let out = match line_result {
                    Ok(line) => {
                        buffer.push_str(&line);
                        // Split on either \r\n or \n so we get one line at a time.
                        let mut events = Vec::new();
                        while let Some(idx) = buffer.find('\n') {
                            let line: String = buffer.drain(..=idx).collect();
                            let trimmed = line.trim_end_matches(&['\r', '\n'][..]).to_string();
                            if trimmed.is_empty() {
                                // Blank line = end of one SSE event.
                                                if !buffer.is_empty() {
                                                    events.push(std::mem::take(buffer));
                                                }
                            } else if let Some(data) = trimmed.strip_prefix("data:") {
                                let payload = data.trim();
                                if !payload.is_empty() {
                                    events.push(payload.to_string());
                                }
                            }
                            // Skip "event:", "id:", "retry:", and ":" comment lines.
                        }
                        futures_util::stream::iter(events.into_iter().map(Ok))
                    }
                    Err(e) => futures_util::stream::iter(std::iter::once(Err(e))),
                };
                Box::new(out) as std::pin::Pin<Box<dyn Stream<Item = Result<String, SdkError>> + Send>>
            })
            .flatten();

        // Map raw JSON strings into ChatChunk values.
        let chunk_stream = event_stream.map(|raw| -> Result<ChatChunk> {
            match raw {
                Ok(json) => {
                    // SSE `data: [DONE]` signals end-of-stream.
                    if json.trim() == "[DONE]" {
                        return Err(SdkError::StreamEnd);
                    }
                    serde_json::from_str::<ChatChunk>(&json).map_err(SdkError::from)
                }
                Err(e) => Err(e),
            }
        })
        // Filter out the end-of-stream marker so consumers just see natural end.
        .filter(|res| !matches!(res, Err(SdkError::StreamEnd)));

        Ok(chunk_stream)
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
