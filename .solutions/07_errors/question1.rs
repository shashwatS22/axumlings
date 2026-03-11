use axum::{extract::Path, response::IntoResponse, routing::get, Router};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("not found")]
struct NotFound;

impl IntoResponse for NotFound {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::NOT_FOUND, "not found").into_response()
    }
}

fn app() -> Router {
    Router::new().route("/item/:id", get(handler))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler(Path(id): Path<u32>) -> Result<String, NotFound> {
    let name = find_item(id)?;
    Ok(name)
}

fn find_item(id: u32) -> Result<String, NotFound> {
    if id == 42 {
        Ok("the answer".to_string())
    } else {
        Err(NotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_found() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/item/42").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), 200);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"the answer");
    }

    #[tokio::test]
    async fn test_not_found() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/item/1").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), 404);
    }
}
