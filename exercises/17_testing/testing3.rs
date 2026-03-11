// Test with State
//
// Test a stateful handler by constructing the real router with real state
// inside the test function.
//
// I AM NOT DONE

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
        // TODO: Create `Arc<AppState>` with count = 42
        // TODO: Pass it to `app()`
        // TODO: Send GET /
        // TODO: Convert body to bytes
        // TODO: Assert body reads "count: 42"
    }
}

fn main() {
    // Run `cargo test --bin testing3` to test your solution!
}
