// Read a custom header: GET /hello with X-User-Name: Bob returns "Hello, Bob!".
//
// Hint: Extract Request and use request.headers().get("X-User-Name").

use axum::{extract::Request, routing::get, Router};

fn app() -> Router {
    Router::new().route("/hello", get(handler))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler(_request: Request) -> String {
    // TODO: Extract X-User-Name from _request.headers() and return format!("Hello, {}!", name)
    "Hello, !".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_header() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/hello")
                    .header("X-User-Name", "Bob")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"Hello, Bob!");
    }
}
