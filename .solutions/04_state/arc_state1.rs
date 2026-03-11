use axum::{extract::State, routing::get, Router};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

fn app() -> Router {
    let state = Arc::new(AtomicUsize::new(0));
    Router::new()
        .route("/count", get(handler))
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
    let count = n.fetch_add(1, Ordering::SeqCst) + 1;
    format!("{}", count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_count() {
        let state = Arc::new(AtomicUsize::new(0));
        let app = Router::new()
            .route("/count", get(handler))
            .with_state(state);
        let response = app
            .oneshot(Request::builder().uri("/count").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"1");
    }
}
