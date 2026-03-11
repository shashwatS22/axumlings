use axum::{extract::Path, routing::get, Router};

fn app() -> Router {
    Router::new().route("/users/:id", get(handler))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler(Path(id): Path<u32>) -> String {
    format!("User {}", id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_path_extractor() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/users/42").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"User 42");
    }
}
