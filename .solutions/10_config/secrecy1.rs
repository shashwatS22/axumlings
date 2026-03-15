// secrecy::Secret
//
// Wrap the API key in Secret<String> so it never appears in logs.
// Use .expose_secret() only when comparing to the request header.
//
// Hint: api_key: secrecy::Secret<String>, cfg.api_key.expose_secret() == header_value

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use secrecy::{ExposeSecret, Secret};
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    api_key: Secret<String>,
}

fn app() -> Router {
    let state = Arc::new(AppState {
        api_key: Secret::new("secret123".into()),
    });
    Router::new()
        .route("/check", get(handler))
        .with_state(state)
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    let key = headers.get("x-api-key").and_then(|v| v.to_str().ok());
    if key != Some(state.api_key.expose_secret().as_str()) {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    "ok".into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_secrecy() {
        let state = Arc::new(AppState {
            api_key: Secret::new("secret123".into()),
        });
        let app = Router::new()
            .route("/check", get(handler))
            .with_state(state);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/check")
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
        assert_eq!(&body[..], b"ok");
    }
}
