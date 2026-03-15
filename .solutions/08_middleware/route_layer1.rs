// route_layer vs layer
//
// Apply auth middleware only to /admin, not to /public.
// Use .route_layer() so it affects only routes defined above it.
//
// Hint: .route("/admin", get(admin)).route_layer(from_fn(auth)).route("/public", get(public)).layer(TraceLayer::new_for_http())

use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use tower_http::trace::TraceLayer;

async fn auth(request: Request, next: Next) -> Response {
    if request.headers().get(header::AUTHORIZATION).is_none() {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    next.run(request).await
}

fn app() -> Router {
    Router::new()
        .route("/admin", get(|| async { "admin" }))
        .route_layer(middleware::from_fn(auth))
        .route("/public", get(|| async { "public" }))
        .layer(TraceLayer::new_for_http())
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
    async fn test_admin_requires_auth() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/admin")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_public_no_auth() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/public")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"public");
    }

    #[tokio::test]
    async fn test_admin_with_auth() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/admin")
                    .header(header::AUTHORIZATION, "Bearer x")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"admin");
    }
}
