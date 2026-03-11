// Use clap to parse CLI args: --port 8080 and --host 0.0.0.0. Start the server with those values.
//
// Hint: #[derive(Parser)] struct Args { #[arg(long, default_value_t = 3000)] port: u16 }

use axum::Router;

fn app() -> Router {
    Router::new().route("/", axum::routing::get(|| async { "ok" }))
}

#[tokio::main]
async fn main() {
    // TODO: Use clap to parse --port and --host
    let port = 3000u16;
    let host = "127.0.0.1";
    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app()).await.unwrap();
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
