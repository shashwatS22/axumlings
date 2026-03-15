// Request ID Middleware
//
// Add x-request-id header to every request and propagate to response.
// Use SetRequestIdLayer + PropagateRequestIdLayer with MakeRequestUuid.
//
// Hint: tower_http::request_id::{SetRequestIdLayer, PropagateRequestIdLayer, MakeRequestUuid}

use axum::{routing::get, Router};
use tower_http::request_id::{
    MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer,
};

fn app() -> Router {
    Router::new()
        .route("/", get(|| async { "ok" }))
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(PropagateRequestIdLayer::x_request_id())
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
    async fn test_request_id() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let id = response.headers().get("x-request-id");
        assert!(id.is_some(), "x-request-id header should be present");
    }
}
