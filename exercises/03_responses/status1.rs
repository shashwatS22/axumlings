// Return status code: POST /items returns 201 Created with body "created".
//
// Hint: Return (StatusCode::CREATED, body).

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};

fn app() -> Router {
    Router::new().route("/items", post(handler))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    // TODO: Return (StatusCode::CREATED, "created")
    (StatusCode::OK, "created")
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_status_created() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/items")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"created");
    }
}
