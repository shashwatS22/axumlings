use axum::Router;
use tower_http::cors::CorsLayer;

fn app() -> Router {
    Router::new()
        .route("/", axum::routing::get(|| async { "ok" }))
        .layer(CorsLayer::permissive())
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
    use axum::http::{Method, Request};
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_cors_preflight() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::OPTIONS)
                    .uri("/")
                    .header("Origin", "https://example.com")
                    .header("Access-Control-Request-Method", "GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert!(response
            .headers()
            .get("access-control-allow-origin")
            .is_some());
    }
}
