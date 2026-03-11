// GET /spawn: spawn a background task that increments a counter; return the count.
// Use Arc<AtomicUsize> and tokio::spawn.
//
// Hint: Spawn background task with tokio::spawn(async move { ... }).

use axum::{extract::State, routing::get, Router};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

fn app() -> Router {
    let state = Arc::new(AtomicUsize::new(0));
    Router::new()
        .route("/spawn", get(handler))
        .with_state(state)
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler(State(n): State<Arc<AtomicUsize>>) -> String {
    // TODO: tokio::spawn a task that does n.fetch_add(1, Ordering::SeqCst); give it n.clone()
    // Then return format!("{}", n.load(Ordering::SeqCst)) (may need a small sleep to let spawn run)
    format!("{}", n.load(Ordering::SeqCst))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_spawn() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/spawn").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"1");
    }
}
