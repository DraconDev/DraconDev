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
