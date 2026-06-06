# dracon-ai-sdk

Lightweight HTTP client for the Dracon AI API.

Consumers depend on this crate instead of the AI library directly. All AI logic lives in the API server (`dracon-platform/ai-api`); this crate is a typed HTTP wrapper.

## Quick start

```toml
[dependencies]
dracon-ai-sdk = { git = "https://github.com/DraconDev/dracon-ai-sdk.git", tag = "v0.1.0" }
```

```rust
use dracon_ai_sdk::{ChatMessage, DraconAi};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ai = DraconAi::from_env()?; // reads AI_API_URL + AI_API_KEY

    let response = ai.chat("free", "my-app", vec![
        ChatMessage::user("Hello!"),
    ]).await?;

    println!("{}", response.content);
    Ok(())
}
```

## Environment variables

| Variable | Required | Description |
|----------|----------|-------------|
| `AI_API_URL` | No | API base URL (default: `http://localhost:3001`) |
| `AI_API_KEY` | Yes | API key for authentication |

## API

```rust
// Chat
let resp: ChatResponse = ai.chat("free", "my-app", messages).await?;

// Chat with full options
let resp = ai.chat_with_options(ChatRequest {
    lane: "writing".into(),
    project_id: "my-app".into(),
    messages,
    model: Some("claude-sonnet".into()),
    max_tokens: Some(2048),
}).await?;

// Streaming
let mut stream = ai.chat_stream("free", "my-app", messages).await?;
while let Some(chunk) = stream.next().await {
    print!("{}", chunk?.content);
}

// Image, Music, SFX, TTS
let image: ImageResponse = ai.image("image", "my-app", "A cat in a hat").await?;
let music: AudioResponse  = ai.music("music", "my-app", "Jazz piano").await?;
let sfx:   AudioResponse  = ai.soundfx("sfx",   "my-app", "Door creak").await?;
let tts:   AudioResponse  = ai.tts("tts",       "my-app", "Hello world").await?;

// Health check
let health: HealthResponse = ai.health().await?;
```

## License

AGPL-3.0
