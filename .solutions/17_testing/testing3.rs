// Test with State
//
// Test a stateful handler by constructing the real router with real state
// inside the test function.
//

use axum::{extract::State, routing::get, Router};
use std::sync::Arc;

struct AppState {
    count: i32,
}

fn app(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/",
            get(|State(s): State<Arc<AppState>>| async move { format!("count: {}", s.count) }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_state() {
        let state = Arc::new(AppState { count: 42 });
        let app = app(state);

        let request = Request::builder().uri("/").body(Body::empty()).unwrap();
        let response = app.oneshot(request).await.unwrap();

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"count: 42");
    }
}

fn main() {
    // Run `cargo test --bin testing3` to test your solution!
}
