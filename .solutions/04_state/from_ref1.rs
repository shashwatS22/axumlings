// FromRef Sub-State
//
// Implement FromRef<AppState> for DbState so handlers can extract State<DbState>.
// DbState holds a connection string; AppState has db: DbState.
//
// Hint: impl FromRef<AppState> for DbState { fn from_ref(s: &AppState) -> Self { s.db.clone() } }

use axum::{extract::FromRef, extract::State, routing::get, Router};

#[derive(Clone)]
struct DbState {
    conn: String,
}

#[derive(Clone)]
struct AppState {
    db: DbState,
}

impl FromRef<AppState> for DbState {
    fn from_ref(s: &AppState) -> Self {
        s.db.clone()
    }
}

fn app() -> Router {
    let state = AppState {
        db: DbState {
            conn: "postgres://localhost".into(),
        },
    };
    Router::new()
        .route("/db", get(handler))
        .with_state(state)
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler(State(db): State<DbState>) -> String {
    format!("conn: {}", db.conn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_from_ref() {
        let state = AppState {
            db: DbState {
                conn: "postgres://test".into(),
            },
        };
        let app = Router::new()
            .route("/db", get(super::handler))
            .with_state(state);
        let response = app
            .oneshot(Request::builder().uri("/db").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"conn: postgres://test");
    }
}
