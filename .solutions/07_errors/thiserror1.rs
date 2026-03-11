use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, body) = match &self {
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
        };
        (status, body).into_response()
    }
}

fn app() -> Router {
    Router::new().route("/user/:id", get(handler))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler(
    axum::extract::Path(id): axum::extract::Path<u32>,
) -> Result<String, AppError> {
    if id == 0 {
        return Err(AppError::BadRequest("id must be positive".into()));
    }
    if id > 100 {
        return Err(AppError::NotFound);
    }
    Ok(format!("user {}", id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_not_found() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/user/101").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_bad_request() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/user/0").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"id must be positive");
    }
}
