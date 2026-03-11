use axum::response::Json;
use axum::{routing::get, Router};
use serde::Serialize;

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

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

async fn handler() -> Json<User> {
    Json(User {
        id: 1,
        name: "Alice".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_json_response() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/user").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], br#"{"id":1,"name":"Alice"}"#);
    }
}
