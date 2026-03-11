use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct UserIn {
    name: String,
    age: u32,
}

#[derive(Debug, Deserialize)]
struct Payload {
    user: UserIn,
}

#[derive(Debug, Serialize)]
struct Response {
    name: String,
    age: u32,
}

fn app() -> Router {
    Router::new().route("/nested", post(handler))
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
        name: p.user.name,
        age: p.user.age,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_nested() {
        let app = app();
        let body = r#"{"user":{"name":"Alice","age":30}}"#;
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/nested")
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
        assert_eq!(json["name"], "Alice");
        assert_eq!(json["age"], 30);
    }
}
