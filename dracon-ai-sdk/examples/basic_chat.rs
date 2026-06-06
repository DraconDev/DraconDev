//! Basic chat example using the SDK.
//!
//! Run with: `AI_API_URL=http://localhost:3001 AI_API_KEY=your-key cargo run --example basic_chat`

use dracon_ai_sdk::{ChatMessage, DraconAi};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ai = DraconAi::from_env()?;

    let response = ai
        .chat("free", "sdk-example", vec![ChatMessage::user("Hello, world!")])
        .await?;

    println!("Model: {}", response.model_used);
    println!("Response: {}", response.content);
    if let Some(usage) = &response.usage {
        println!(
            "Tokens: {} prompt + {} completion = {} total",
            usage.prompt_tokens, usage.completion_tokens, usage.total_tokens
        );
    }

    Ok(())
}
