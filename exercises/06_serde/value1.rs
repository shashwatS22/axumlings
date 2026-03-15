// serde_json::Value
//
// Parse unknown JSON into serde_json::Value and extract the "name" field.
// Return Some(name) if present and a string, None otherwise.
//
// Hint: let v: Value = serde_json::from_str(s).ok()?; v["name"].as_str().map(|s| s.to_string())

use axum::{routing::post, Json, Router};
use serde::Serialize;
use serde_json::Value;

fn app() -> Router {
    Router::new().route("/extract", post(handler))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

fn extract_name(s: &str) -> Option<String> {
    // TODO: Parse s as Value, get v["name"].as_str(), return Some(name.to_string())
    let _ = s;
    None
}

#[derive(Serialize)]
struct Response {
    name: Option<String>,
}

async fn handler(body: String) -> Json<Response> {
    Json(Response {
        name: extract_name(&body),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_extract_name() {
        let app = app();
        let body = r#"{"name":"Bob","age":30}"#;
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/extract")
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
    }

    #[tokio::test]
    async fn test_extract_missing() {
        let app = app();
        let body = r#"{"age":30}"#;
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/extract")
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
        assert!(json["name"].is_null());
    }
}
