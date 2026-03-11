// Structured logging: log each request with tracing::info! including method and path.
//
// Hint: tracing::info!(method = %req.method(), path = %req.uri().path(), "request")

use axum::{extract::Request, routing::get, Router};

fn app() -> Router {
    Router::new()
        .route("/", get(handler))
        // TODO: Add middleware or wrap handler to log requests
}

async fn handler(_req: Request) -> &'static str {
    // TODO: tracing::info!(method = %_req.method(), path = %_req.uri().path(), "request");
    "ok"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
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
    async fn test_handler() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), 200);
    }
}
