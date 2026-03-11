// oneshot Handler Test
//
// Test an Axum handler directly without running a TCP server.
// `ServiceExt::oneshot` sends a single request to the router and returns the response.
//

use axum::{routing::get, Router};

fn app() -> Router {
    Router::new().route("/", get(|| async { "Hello!" }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_hello() {
        let app = app();

        let request = Request::builder().uri("/").body(Body::empty()).unwrap();
        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), 200);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Hello!");
    }
}

fn main() {
    // Run `cargo test --bin testing2` to test your solution!
}
