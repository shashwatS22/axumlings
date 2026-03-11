// Return a custom error from a handler; implement IntoResponse for your error type.
//
// Hint: impl IntoResponse for Error { fn into_response(self) -> Response { ... } }

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

#[derive(Debug)]
struct AppError {
    message: String,
}

// TODO: impl IntoResponse for AppError: return (StatusCode::INTERNAL_SERVER_ERROR, self.message)
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::OK, "").into_response() // placeholder: change to 500 and self.message
    }
}

fn app() -> Router {
    Router::new().route("/error", get(handler))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler() -> Result<&'static str, AppError> {
    Err(AppError {
        message: "something went wrong".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_error() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/error").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"something went wrong");
    }
}
