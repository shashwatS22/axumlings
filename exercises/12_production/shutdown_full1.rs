// Graceful Shutdown (Full Pattern)
//
// Serve with graceful shutdown on Ctrl+C. Use with_graceful_shutdown(shutdown_signal()).
// The server stops accepting new connections and drains existing ones.
//
// Hint: axum::serve(listener, app()).with_graceful_shutdown(shutdown_signal()).await

use axum::Router;

fn app() -> Router {
    Router::new().route("/", axum::routing::get(|| async { "ok" }))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    // TODO: axum::serve(listener, app()).with_graceful_shutdown(shutdown_signal()).await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}

// TODO: async fn shutdown_signal() { tokio::signal::ctrl_c().await.ok(); }

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
