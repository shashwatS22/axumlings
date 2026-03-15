// Untagged Enum
//
// Deserialize JSON without a type discriminator. Add #[serde(untagged)] so serde tries
// each variant in order. Put more specific variants first.
//
// Hint: #[serde(untagged)] on the enum

use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Payload {
    Name { name: String },
    Id { id: u32 },
}

#[derive(Debug, Serialize)]
struct Response {
    kind: String,
    value: String,
}

fn app() -> Router {
    Router::new().route("/parse", post(handler))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler(Json(p): Json<Payload>) -> Json<Response> {
    let (kind, value) = match p {
        Payload::Name { name } => ("name".into(), name),
        Payload::Id { id } => ("id".into(), id.to_string()),
    };
    Json(Response { kind, value })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_name() {
        let app = app();
        let body = r#"{"name":"Alice"}"#;
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/parse")
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
        assert_eq!(json["kind"], "name");
        assert_eq!(json["value"], "Alice");
    }

    #[tokio::test]
    async fn test_id() {
        let app = app();
        let body = r#"{"id":42}"#;
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/parse")
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
        assert_eq!(json["kind"], "id");
        assert_eq!(json["value"], "42");
    }
}
