// Atomic counter: GET /inc returns the current count and increments it.
//
// Hint: Use Arc::new(AtomicUsize::new(0)).

use axum::{extract::State, routing::get, Router};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

fn app() -> Router {
    let state = Arc::new(AtomicUsize::new(0));
    Router::new()
        .route("/inc", get(handler))
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
    // TODO: Return current count and increment: n.fetch_add(1, Ordering::SeqCst)
    format!("{}", n.load(Ordering::SeqCst))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_atomic_inc() {
        let state = Arc::new(AtomicUsize::new(0));
        let app = Router::new()
            .route("/inc", get(handler))
            .with_state(state);
        let r1 = app
            .clone()
            .oneshot(Request::builder().uri("/inc").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let b1 = axum::body::to_bytes(r1.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&b1[..], b"0");
        let r2 = app
            .oneshot(Request::builder().uri("/inc").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let b2 = axum::body::to_bytes(r2.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&b2[..], b"1");
    }
}
