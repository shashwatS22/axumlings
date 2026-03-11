// Deserialize with optional field and default: { "name": "Bob" } -> nickname defaults to "unknown".
//
// Hint: #[serde(default)] for Option or default value.

use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Payload {
    name: String,
    // TODO: Add nickname: Option<String> so missing "nickname" deserializes to None
}

#[derive(Debug, Serialize)]
struct Response {
    name: String,
    nickname: String,
}

fn app() -> Router {
    Router::new().route("/user", post(handler))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler(Json(p): Json<Payload>) -> Json<Response> {
    // TODO: Add nickname field to Payload; return nickname as p.nickname.unwrap_or("unknown")
    Json(Response {
        name: p.name,
        nickname: "unknown".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_default_nickname() {
        let app = app();
        let body = r#"{"name":"Bob"}"#;
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/user")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(json["name"], "Bob");
        assert_eq!(json["nickname"], "unknown");
    }

    #[tokio::test]
    async fn test_nickname_provided() {
        let app = app();
        let body = r#"{"name":"Bob","nickname":"Bobby"}"#;
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/user")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(json["nickname"], "Bobby");
    }
}
