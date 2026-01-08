use std::collections::HashMap;

use axum::{Json, Router, extract::State, routing::get};
use rust_decimal::{Decimal, dec};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::error::Result;

#[derive(Debug, Default, Serialize, Deserialize)]
struct BudgetSummary {
    category_totals: HashMap<String, Decimal>,
    grand_total: Decimal,
}

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/summary", get(summary))
        .with_state(pool)
}

async fn summary(State(pool): State<PgPool>) -> Result<Json<BudgetSummary>> {
    // TODO: total per category and total total
    let category_totals = sqlx::query!(
        "
        SELECT category, SUM(amount)
        FROM budget
        WHERE category IS NOT NULL
        GROUP BY category;
    ",
    )
    .fetch_all(&pool)
    .await?
    .into_iter()
    .filter_map(|row| {
        if let Some(sum) = row.sum {
            Some((row.category, sum))
        } else {
            None
        }
    })
    .collect();
    let grand_total = sqlx::query!("SELECT SUM(amount) FROM budget")
        .fetch_one(&pool)
        .await?
        .sum
        .unwrap_or(dec!(0));
    Ok(Json(BudgetSummary {
        category_totals,
        grand_total,
    }))
}
