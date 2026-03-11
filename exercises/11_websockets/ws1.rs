// WebSocket handler: GET /ws upgrades to WebSocket and echoes messages back.
//
// Hint: axum::extract::ws::WebSocketUpgrade, ws.on_upgrade(handler)

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::get,
    Router,
};

fn app() -> Router {
    Router::new().route("/ws", get(handler))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler(ws: WebSocketUpgrade) -> Response {
    // TODO: ws.on_upgrade(socket_handler)
    ws.on_upgrade(|_socket| async {})
}

async fn socket_handler(mut socket: WebSocket) {
    // TODO: while let Some(Ok(msg)) = socket.recv().await { socket.send(msg).await.ok(); }
    while let Some(Ok(_msg)) = socket.recv().await {
        // echo back
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_ws_route() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/ws")
                    .header("Upgrade", "websocket")
                    .header("Connection", "Upgrade")
                    .header("Sec-WebSocket-Key", "dGhlIHNhbXBsZSBub25jZQ==")
                    .header("Sec-WebSocket-Version", "13")
                    .header("Sec-WebSocket-Extensions", "")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), 404, "WebSocket route should be registered");
    }
}
