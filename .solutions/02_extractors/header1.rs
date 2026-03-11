use axum::{
    extract::Request,
    routing::get,
    Router,
};

fn app() -> Router {
    Router::new().route("/hello", get(handler))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler(request: Request) -> String {
    let name = request
        .headers()
        .get("X-User-Name")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("Guest");
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_header() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/hello")
                    .header("X-User-Name", "Bob")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"Hello, Bob!");
    }
}
