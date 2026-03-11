use axum::Router;
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = 3000)]
    port: u16,
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
}

fn app() -> Router {
    Router::new().route("/", axum::routing::get(|| async { "ok" }))
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let addr = format!("{}:{}", args.host, args.port);
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
