// Multi-Segment Path
//
// Extract multiple dynamic URL segments: GET /orgs/:org/repos/:repo
// Use Path((org, repo)): Path<(String, String)> — tuple order matches URL order.
//
// Hint: Path((org, repo)): Path<(String, String)>

use axum::{extract::Path, routing::get, Router};

fn app() -> Router {
    Router::new().route("/orgs/:org/repos/:repo", get(handler))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler(Path((org, repo)): Path<(String, String)>) -> String {
    format!("org={} repo={}", org, repo)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_multi_segment_path() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/orgs/acme/repos/widget")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"org=acme repo=widget");
    }
}
