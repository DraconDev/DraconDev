//! Public types for the Dracon AI SDK.
//!
//! These mirror the HTTP API contract — see `dracon-platform/ai-api/docs/API-CONTRACT.md`.

use serde::{Deserialize, Serialize};

/// A chat message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

impl ChatMessage {
    /// Create a user message.
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
        }
    }

    /// Create an assistant message.
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
        }
    }

    /// Create a system message.
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: Role::System,
            content: content.into(),
        }
    }
}

/// The role of a message in a conversation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

/// A chat completion request.
#[derive(Debug, Clone, Serialize)]
pub struct ChatRequest {
    pub lane: String,
    pub project_id: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

/// A chat completion response.
#[derive(Debug, Clone, Deserialize)]
pub struct ChatResponse {
    pub content: String,
    pub model_used: String,
    #[serde(default)]
    pub finish_reason: Option<String>,
    #[serde(default)]
    pub usage: Option<UsageStats>,
}

/// Token usage statistics.
#[derive(Debug, Clone, Deserialize)]
pub struct UsageStats {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// A single chunk from a streaming chat response.
#[derive(Debug, Clone, Deserialize)]
pub struct ChatChunk {
    pub content: String,
    pub model: String,
    #[serde(default)]
    pub finish_reason: Option<String>,
}

/// An image generation request.
#[derive(Debug, Clone, Serialize)]
pub struct ImageRequest {
    pub lane: String,
    pub project_id: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

/// An image generation response.
#[derive(Debug, Clone, Deserialize)]
pub struct ImageResponse {
    pub url: String,
    pub model_used: String,
}

/// A text-to-speech request.
#[derive(Debug, Clone, Serialize)]
pub struct TtsRequest {
    pub lane: String,
    pub project_id: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

/// A music generation request.
#[derive(Debug, Clone, Serialize)]
pub struct MusicRequest {
    pub lane: String,
    pub project_id: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

/// A sound effects request.
#[derive(Debug, Clone, Serialize)]
pub struct SoundFxRequest {
    pub lane: String,
    pub project_id: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

/// An audio response (TTS, music, or SFX).
#[derive(Debug, Clone, Deserialize)]
pub struct AudioResponse {
    pub url: String,
    pub model_used: String,
    #[serde(default)]
    pub duration_ms: Option<u64>,
}

/// Health check response.
#[derive(Debug, Clone, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub lanes: Vec<String>,
    #[serde(default)]
    pub providers: serde_json::Value,
    #[serde(default)]
    pub stats: serde_json::Value,
}
