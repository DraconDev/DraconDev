//! Error types for the SDK.

use thiserror::Error;

/// Errors that can occur when using the SDK.
#[derive(Debug, Error)]
pub enum SdkError {
    /// The API server returned an error response.
    #[error("API error: {0}")]
    Api(String),

    /// The API returned an unexpected status code.
    #[error("HTTP {status}: {message}")]
    Http {
        status: u16,
        message: String,
    },

    /// Authentication failed (401).
    #[error("authentication failed: invalid or missing API key")]
    Unauthorized,

    /// Rate limit exceeded (429).
    #[error("rate limit exceeded — try again later")]
    RateLimited,

    /// The server is temporarily unavailable (503).
    #[error("service temporarily unavailable")]
    ServiceUnavailable,

    /// Upstream provider failed (502).
    #[error("upstream AI provider failed")]
    BadGateway,

    /// Request timed out (504).
    #[error("request timed out")]
    Timeout,

    /// Network/transport error.
    #[error("transport error: {0}")]
    Transport(#[from] reqwest::Error),

    /// JSON deserialization error.
    #[error("failed to parse response: {0}")]
    Parse(#[from] serde_json::Error),

    /// Missing required environment variable.
    #[error("missing environment variable: {0}")]
    MissingEnv(String),

    /// Bad request (400).
    #[error("bad request: {0}")]
    BadRequest(String),

    /// Network/transport error during streaming.
    #[error("transport error: {0}")]
    Http(String),

    /// Internal sentinel — `data: [DONE]` from SSE. Consumers never see this
    /// because `chat_stream` filters it out before yielding items.
    #[doc(hidden)]
    #[error("stream ended")]
    StreamEnd,
}

impl SdkError {
    /// Classify an HTTP status code into a typed error.
    pub fn from_status(status: u16, body: &str) -> Self {
        match status {
            400 => SdkError::BadRequest(body.to_string()),
            401 => SdkError::Unauthorized,
            429 => SdkError::RateLimited,
            502 => SdkError::BadGateway,
            503 => SdkError::ServiceUnavailable,
            504 => SdkError::Timeout,
            _ => SdkError::Http {
                status,
                message: body.to_string(),
            },
        }
    }
}

pub type Result<T> = std::result::Result<T, SdkError>;
