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

async fn handler(Query(params): Query<HashMap<String, String>>) -> String {
    let q = params.get("q").map(|s| s.as_str()).unwrap_or("");
    format!("Search: {}", q)
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
