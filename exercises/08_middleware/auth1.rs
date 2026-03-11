// Auth middleware: reject requests without Authorization: Bearer <token>; otherwise call the handler.
//
// Hint: Check header before handler; use axum::middleware or a layer that checks the request.

use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
    routing::get,
    Router,
};

async fn auth_middleware(request: Request, next: Next) -> Response {
    // TODO: If request.headers().get(header::AUTHORIZATION) is not "Bearer secret", return 401
    next.run(request).await
}

fn app() -> Router {
    Router::new()
        .route("/protected", get(|| async { "secret data" }))
        // TODO: .route_layer(axum::middleware::from_fn(auth_middleware))
        .route("/public", get(|| async { "hello" }))
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
    async fn test_protected_no_auth() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/protected")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_protected_with_auth() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/protected")
                    .header(header::AUTHORIZATION, "Bearer secret")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), 200);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"secret data");
    }
}
