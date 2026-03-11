// Deserialize a tagged enum: POST /event accepts { "type": "Click", "x": 10, "y": 20 }.
//
// Hint: #[serde(tag = "type")] for the enum variant name.

use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

// TODO: Add #[serde(tag = "type")] so JSON {"type":"Click","x":10,"y":20} deserializes
#[derive(Debug, Deserialize, Serialize)]
enum Event {
    Click { x: u32, y: u32 },
    Key { key: String },
}

fn app() -> Router {
    Router::new().route("/event", post(handler))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler(Json(e): Json<Event>) -> Json<String> {
    // TODO: match e and return format!("click {} {}", x, y) or format!("key {}", key)
    let s = match e {
        Event::Click { x, y } => format!("click {} {}", x, y),
        Event::Key { key } => format!("key {}", key),
    };
    Json(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_click() {
        let app = app();
        let body = r#"{"type":"Click","x":10,"y":20}"#;
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/event")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&bytes[..], b"\"click 10 20\"");
    }
}
