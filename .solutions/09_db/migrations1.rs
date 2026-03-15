// Migrations
//
// Run SQL migrations at startup using sqlx::migrate!.
// Create migrations/0001_create_items.sql and run migrate at startup.
//
// Hint: sqlx::migrate!("./migrations").run(&pool).await

use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, FromRow, Serialize)]
struct Item {
    id: i64,
    name: String,
}

fn app(pool: SqlitePool) -> Router {
    Router::new()
        .route("/items", get(handler))
        .with_state(pool)
}

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app(pool)).await.unwrap();
}

async fn handler(State(pool): State<SqlitePool>) -> Json<Vec<Item>> {
    let rows = sqlx::query_as::<_, Item>("SELECT id, name FROM items")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();
    Json(rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_migrations() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        sqlx::query("INSERT INTO items (id, name) VALUES (1, 'Widget')")
            .execute(&pool)
            .await
            .unwrap();

        let app = app(pool);
        let response = app
            .oneshot(Request::builder().uri("/items").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(json.is_array());
        assert_eq!(json[0]["name"], "Widget");
    }
}
