use axum::Router;

fn app() -> Router {
    Router::new().route("/", axum::routing::get(|| async { "ok" }))
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.unwrap();
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    let server = axum::serve(listener, app());
    server.with_graceful_shutdown(shutdown_signal()).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_app() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), 200);
    }
}
