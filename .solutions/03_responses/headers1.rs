use axum::{
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};

fn app() -> Router {
    Router::new().route("/custom", get(handler))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        "X-Custom",
        HeaderValue::from_static("value"),
    );
    (StatusCode::OK, headers, "ok")
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_custom_header() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/custom").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let val = response.headers().get("X-Custom").and_then(|v| v.to_str().ok());
        assert_eq!(val, Some("value"));
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"ok");
    }
}
