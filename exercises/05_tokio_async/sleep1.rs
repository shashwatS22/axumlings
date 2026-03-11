// GET /delay: wait 100ms then return "done". Use tokio::time::sleep.
//
// Hint: sleep(Duration::from_millis(100)).await

use axum::{routing::get, Router};

fn app() -> Router {
    Router::new().route("/delay", get(handler))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler() -> &'static str {
    // TODO: tokio::time::sleep(Duration::from_millis(100)).await then return "done"
    "done"
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use std::time::Instant;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_delay() {
        let app = app();
        let start = Instant::now();
        let response = app
            .oneshot(Request::builder().uri("/delay").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() >= 50);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"done");
    }
}
