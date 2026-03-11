// Parse JSON body: POST /users with {"name": "Alice"} returns "Created: Alice".
//
// Hint: Use Json(payload): Json<CreateUser> with #[derive(Deserialize)] struct.

use axum::{extract::Json, routing::post, Router};
use serde::Deserialize;

fn app() -> Router {
    Router::new().route("/users", post(handler))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
}

async fn handler(Json(payload): Json<CreateUser>) -> String {
    // TODO: Return format!("Created: {}", payload.name)
    "Created: ".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_json_extractor() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/users")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"name":"Alice"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"Created: Alice");
    }
}
