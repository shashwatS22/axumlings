use axum::Router;
use std::time::Duration;
use tower::limit::ConcurrencyLimitLayer;

fn app() -> Router {
    Router::new()
        .route("/slow", axum::routing::get(slow_handler))
        .layer(ConcurrencyLimitLayer::new(2))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn slow_handler() -> &'static str {
    tokio::time::sleep(Duration::from_millis(50)).await;
    "done"
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_limit() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/slow").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), 200);
    }
}
