use dracon_ai_sdk::{ChatMessage, ChatRequest, DraconAi, Role};
use futures_util::StreamExt;

#[test]
fn chat_message_constructors() {
    let user = ChatMessage::user("hello");
    assert_eq!(user.role, Role::User);
    assert_eq!(user.content, "hello");

    let assistant = ChatMessage::assistant("hi back");
    assert_eq!(assistant.role, Role::Assistant);
    assert_eq!(assistant.content, "hi back");

    let system = ChatMessage::system("be helpful");
    assert_eq!(system.role, Role::System);
    assert_eq!(system.content, "be helpful");
}

#[test]
fn dracon_ai_new_constructs() {
    let _ai = DraconAi::new("http://localhost:3001", "key");
    // No panic means it constructed fine
}

#[test]
fn dracon_ai_new_strips_trailing_slash() {
    let _ai = DraconAi::new("http://localhost:3001/", "key");
    // No panic means it constructed fine
}

#[tokio::test]
async fn health_returns_error_when_no_server() {
    // No server running on 127.0.0.1:1 — should fail with transport error
    let ai = DraconAi::new("http://127.0.0.1:1", "test-key");
    let result = ai.health().await;
    assert!(result.is_err());
}

#[test]
fn role_serialization() {
    let json = serde_json::to_string(&Role::User).unwrap();
    assert_eq!(json, "\"user\"");

    let json = serde_json::to_string(&Role::Assistant).unwrap();
    assert_eq!(json, "\"assistant\"");

    let json = serde_json::to_string(&Role::System).unwrap();
    assert_eq!(json, "\"system\"");
}

#[test]
fn chat_request_skips_none_fields() {
    use dracon_ai_sdk::ChatRequest;
    let req = ChatRequest {
        lane: "free".to_string(),
        project_id: "test".to_string(),
        messages: vec![ChatMessage::user("hi")],
        model: None,
        max_tokens: None,
    };
    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("\"lane\":\"free\""));
    assert!(json.contains("\"project_id\":\"test\""));
    assert!(!json.contains("\"model\""));
    assert!(!json.contains("\"max_tokens\""));
}

/// Spawn a minimal HTTP server that responds to POST with a fixed SSE
/// payload, then return the bound port.
async fn spawn_sse_server(payload: &'static str) -> u16 {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let port = listener.local_addr().expect("addr").port();

    tokio::spawn(async move {
        loop {
            let (mut stream, _) = match listener.accept().await {
                Ok(p) => p,
                Err(_) => return,
            };
            tokio::spawn(async move {
                // Read the request line + headers (we don't actually parse them
                // for this test, just consume them).
                let mut buf = [0u8; 2048];
                let _ = stream.read(&mut buf).await;

                let body = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    payload.len(),
                    payload
                );
                let _ = stream.write_all(body.as_bytes()).await;
                let _ = stream.shutdown().await;
            });
        }
    });

    port
}

#[tokio::test]
async fn chat_stream_parses_sse_chunks() {
    // Realistic SSE payload from the server: one event per chunk, blank lines
    // separate events, and `[DONE]` signals end-of-stream.
    let payload = "\
data: {\"content\": \"hello \", \"model\": \"m\", \"finish_reason\": null}

data: {\"content\": \"world\", \"model\": \"m\", \"finish_reason\": \"stop\"}

data: [DONE]

";
    let port = spawn_sse_server(payload).await;
    let ai = DraconAi::new(format!("http://127.0.0.1:{}", port), "test-key");

    let req = ChatRequest {
        lane: "free".to_string(),
        project_id: "test".to_string(),
        messages: vec![ChatMessage::user("hi")],
        model: None,
        max_tokens: None,
    };

    let mut stream = ai
        .chat_stream_with_options(req)
        .await
        .expect("stream request should succeed");

    let mut chunks = Vec::new();
    while let Some(item) = stream.next().await {
        chunks.push(item.expect("chunk should be Ok"));
    }

    assert_eq!(chunks.len(), 2, "expected 2 chunks before [DONE]");
    assert_eq!(chunks[0].content, "hello ");
    assert_eq!(chunks[0].model, "m");
    assert!(chunks[0].finish_reason.is_none());
    assert_eq!(chunks[1].content, "world");
    assert_eq!(chunks[1].finish_reason.as_deref(), Some("stop"));
}

#[tokio::test]
async fn chat_stream_handles_crlf_and_comments() {
    // SSE allows \r\n line endings and `: comment` lines. Verify we tolerate
    // both without dropping or duplicating chunks.
    let payload = ": keep-alive\r\n\
data: {\"content\": \"a\", \"model\": \"m\"}\r\n\r\n\
: another comment\n\
data: {\"content\": \"b\", \"model\": \"m\", \"finish_reason\": null}\n\n\
data: [DONE]\n";
    let port = spawn_sse_server(payload).await;
    let ai = DraconAi::new(format!("http://127.0.0.1:{}", port), "test-key");

    let req = ChatRequest {
        lane: "free".to_string(),
        project_id: "test".to_string(),
        messages: vec![ChatMessage::user("hi")],
        model: None,
        max_tokens: None,
    };

    let mut stream = ai
        .chat_stream_with_options(req)
        .await
        .expect("stream request should succeed");
    let mut chunks = Vec::new();
    while let Some(item) = stream.next().await {
        chunks.push(item.expect("chunk should be Ok"));
    }

    assert_eq!(chunks.len(), 2);
    assert_eq!(chunks[0].content, "a");
    assert_eq!(chunks[1].content, "b");
}
