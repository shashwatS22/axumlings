// skip_serializing_if
//
// Omit Option fields from JSON when None. Add #[serde(skip_serializing_if = "Option::is_none")]
// to optional fields so they are absent from output instead of null.
//
// Hint: #[serde(skip_serializing_if = "Option::is_none")] on each Option<T> field

use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct User {
    id: u32,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
}

fn app() -> Router {
    Router::new().route("/user", get(handler))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler() -> Json<User> {
    Json(User {
        id: 1,
        name: "Alice".into(),
        email: None,
        phone: Some("555-1234".into()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_skip_none() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/user").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(json["id"], 1);
        assert_eq!(json["name"], "Alice");
        assert!(json.get("email").is_none(), "email should be absent when None");
        assert_eq!(json["phone"], "555-1234");
    }
}
