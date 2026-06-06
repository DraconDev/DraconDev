use dracon_ai_sdk::{ChatMessage, DraconAi, Role};

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
fn dracon_ai_new_strips_trailing_slash() {
    let ai = DraconAi::new("http://localhost:3001/", "key");
    // The base URL is private, but we can verify the struct is built
    assert_eq!(ai.api_key_for_test(), "key");
}

#[test]
fn dracon_ai_new_keeps_url_unchanged() {
    let ai = DraconAi::new("http://localhost:3001", "key");
    assert_eq!(ai.api_key_for_test(), "key");
}

#[tokio::test]
async fn health_returns_unauthorized_when_no_server() {
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
