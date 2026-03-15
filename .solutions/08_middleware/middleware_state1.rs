// Middleware with State
//
// Use middleware::from_fn_with_state to pass AppState into middleware.
// Reject requests where X-Api-Key != state.api_key.
//
// Hint: async fn(State(s): State<Arc<AppState>>, req: Request, next: Next) -> Response

use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use std::sync::Arc;

struct AppState {
    api_key: String,
}

async fn check_key(
    State(s): State<Arc<AppState>>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let key = request
        .headers()
        .get("x-api-key")
        .and_then(|v| v.to_str().ok());
    if key != Some(s.api_key.as_str()) {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    next.run(request).await
}

fn app() -> Router {
    let state = Arc::new(AppState {
        api_key: "secret123".into(),
    });
    Router::new()
        .route("/api/data", get(|| async { "data" }))
        .route_layer(middleware::from_fn_with_state(state.clone(), check_key))
        .with_state(state)
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_no_key() {
        let state = Arc::new(AppState {
            api_key: "secret123".into(),
        });
        let app = Router::new()
            .route("/api/data", get(|| async { "data" }))
            .route_layer(middleware::from_fn_with_state(state.clone(), check_key))
            .with_state(state);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/data")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_valid_key() {
        let state = Arc::new(AppState {
            api_key: "secret123".into(),
        });
        let app = Router::new()
            .route("/api/data", get(|| async { "data" }))
            .route_layer(middleware::from_fn_with_state(state.clone(), check_key))
            .with_state(state);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/data")
                    .header("x-api-key", "secret123")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"data");
    }
}
