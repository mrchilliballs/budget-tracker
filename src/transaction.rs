use crate::error::Result;
use axum::{Json, Router, extract::State, routing::get};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::Date;

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(get_transactions))
        .with_state(pool)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Transaction {
    id: i64,
    description: String,
    amount: Decimal,
    category: String,
    date: Date,
}

async fn get_transactions(State(pool): State<PgPool>) -> Result<Json<Vec<Transaction>>> {
    let rows = sqlx::query_as!(Transaction, "SELECT * FROM budget")
        .fetch_all(&pool)
        .await?;
    Ok(Json(rows))
}
