// Extract query params: GET /search?q=hello should return "Search: hello".
//
// Hint: Use Query(params): Query<HashMap<String, String>> or a derived struct.

use axum::{extract::Query, routing::get, Router};
use std::collections::HashMap;

fn app() -> Router {
    Router::new().route("/search", get(handler))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler(Query(_params): Query<HashMap<String, String>>) -> String {
    // TODO: Get "q" from params and return format!("Search: {}", q)
    "Search: ".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_query_extractor() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/search?q=hello")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"Search: hello");
    }
}
