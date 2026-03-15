// Body Size Limit
//
// Limit request body size to 1MB using DefaultBodyLimit.
// Returns 413 Payload Too Large for oversized requests.
//
// Hint: Add a layer to limit body size. 1MB = 1024 * 1024 bytes.

use axum::{routing::post, Router};

fn app() -> Router {
    Router::new()
        .route("/upload", post(handler))
        // TODO: Add a body limit layer (1MB max)
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler(body: String) -> String {
    format!("received {} bytes", body.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_small_body() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/upload")
                    .body(Body::from("hello"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"received 5 bytes");
    }

    #[tokio::test]
    async fn test_large_body_rejected() {
        let app = app();
        let large = vec![0u8; 2 * 1024 * 1024]; // 2MB
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/upload")
                    .header("content-length", large.len())
                    .body(Body::from(large))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    }
}
