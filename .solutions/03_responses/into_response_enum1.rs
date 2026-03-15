// IntoResponse Enum
//
// Implement IntoResponse for ApiResponse so handlers can return it directly.
// Match each variant: Ok -> 200, Created -> 201, Data(v) -> 200 + Json(v).
//
// Hint: impl IntoResponse for ApiResponse { fn into_response(self) -> Response { match self { ... } } }

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::Serialize;

#[derive(Debug)]
enum ApiResponse {
    Ok,
    Created,
    Data(Item),
}

#[derive(Debug, Serialize)]
struct Item {
    id: u32,
    name: String,
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            ApiResponse::Ok => StatusCode::OK.into_response(),
            ApiResponse::Created => StatusCode::CREATED.into_response(),
            ApiResponse::Data(v) => (StatusCode::OK, Json(v)).into_response(),
        }
    }
}

fn app() -> Router {
    Router::new()
        .route("/ok", get(|| async { ApiResponse::Ok }))
        .route("/created", get(|| async { ApiResponse::Created }))
        .route("/data", get(|| async {
            ApiResponse::Data(Item {
                id: 1,
                name: "test".into(),
            })
        }))
}

#[tokio::main]
async fn main() {
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
    async fn test_ok() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/ok").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_created() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/created").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_data() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/data").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["id"], 1);
        assert_eq!(json["name"], "test");
    }
}
