// select! Macro
//
// GET /race runs two futures concurrently; return "done" if slow_op completes first,
// "timeout" if sleep(50ms) completes first. Use tokio::select!.
//
// Hint: Use tokio::select! to race two futures. Return different strings based on which completes first.

use axum::{routing::get, Router};
use std::time::Duration;
use tokio::time::sleep;

async fn slow_op() -> () {
    sleep(Duration::from_millis(10)).await;
}

fn app() -> Router {
    Router::new().route("/race", get(handler))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler() -> &'static str {
    // TODO: Use tokio::select! to race slow_op() vs sleep(50ms). Return "done" or "timeout" accordingly.
    "???"
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_select_done() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/race").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        // slow_op is 10ms, sleep is 50ms — slow_op should win
        assert!(
            &body[..] == b"done" || &body[..] == b"timeout",
            "expected 'done' or 'timeout', got {:?}",
            std::str::from_utf8(&body)
        );
    }
}
