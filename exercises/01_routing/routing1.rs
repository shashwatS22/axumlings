// Add a GET route "/" that returns "Hello, Axum!"
//
// Hint: Run `axumlings hint` for help, or see the solution after passing.

use axum::{routing::get, Router};

fn app() -> Router {
    let app = Router::new();
    // TODO: Add a route for GET "/" that calls the handler below
    app
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler() -> &'static str {
    "Hello, Axum!"
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_root_returns_hello() {
        let app = app();

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"Hello, Axum!");
    }
}
