use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Payload {
    #[serde(rename = "userName")]
    user_name: String,
}

#[derive(Debug, Serialize)]
struct Response {
    #[serde(rename = "userName")]
    user_name: String,
}

fn app() -> Router {
    Router::new().route("/user", post(handler))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler(Json(p): Json<Payload>) -> Json<Response> {
    Json(Response {
        user_name: p.user_name,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_rename() {
        let app = app();
        let body = r#"{"userName":"Alice"}"#;
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/user")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(json["userName"], "Alice");
    }
}
