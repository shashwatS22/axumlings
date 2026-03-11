// oneshot Handler Test
//
// Test an Axum handler directly without running a TCP server.
// `ServiceExt::oneshot` sends a single request to the router and returns the response.
//
// I AM NOT DONE

use axum::{routing::get, Router};

fn app() -> Router {
    Router::new().route("/", get(|| async { "Hello!" }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    // TODO: use tower::ServiceExt;
    // TODO: use http_body_util::BodyExt;

    #[tokio::test]
    async fn test_hello() {
        let app = app();

        // TODO: Create a Request::builder().uri("/").body(Body::empty()).unwrap()
        // TODO: Send it to `app` using `.oneshot(request).await.unwrap()`
        // TODO: Assert status code is 200 OK
        // TODO: Convert body to bytes using `.into_body().collect().await.unwrap().to_bytes()`
        // TODO: Assert body equals `b"Hello!"` as a slice
    }
}

fn main() {
    // Run `cargo test --bin testing2` to test your solution!
}
