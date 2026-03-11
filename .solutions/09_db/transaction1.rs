use axum::{extract::State, routing::post, Json, Router};
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Debug, Deserialize)]
struct TransferRequest {
    from: i64,
    to: i64,
    amount: i64,
}

fn app(pool: SqlitePool) -> Router {
    Router::new()
        .route("/transfer", post(handler))
        .with_state(pool)
}

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::query("CREATE TABLE accounts (id INTEGER PRIMARY KEY, balance INTEGER NOT NULL)")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO accounts (id, balance) VALUES (1, 100), (2, 50)")
        .execute(&pool)
        .await
        .unwrap();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app(pool)).await.unwrap();
}

async fn handler(
    State(pool): State<SqlitePool>,
    Json(req): Json<TransferRequest>,
) -> &'static str {
    let mut tx = pool.begin().await.unwrap();
    sqlx::query("UPDATE accounts SET balance = balance - ? WHERE id = ?")
        .bind(req.amount)
        .bind(req.from)
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query("UPDATE accounts SET balance = balance + ? WHERE id = ?")
        .bind(req.amount)
        .bind(req.to)
        .execute(&mut *tx)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    "ok"
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_transfer() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::query("CREATE TABLE accounts (id INTEGER PRIMARY KEY, balance INTEGER NOT NULL)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO accounts (id, balance) VALUES (1, 100), (2, 50)")
            .execute(&pool)
            .await
            .unwrap();

        let app = app(pool.clone());
        let body = r#"{"from":1,"to":2,"amount":30}"#;
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/transfer")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), 200);

        let row: (i64,) = sqlx::query_as("SELECT balance FROM accounts WHERE id = 1")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(row.0, 70);
        let row: (i64,) = sqlx::query_as("SELECT balance FROM accounts WHERE id = 2")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(row.0, 80);
    }
}
