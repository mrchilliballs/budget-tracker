use crate::error::Result;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::{HeaderName, StatusCode, header},
    response::{IntoResponse, Response},
    routing::get,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::PrimitiveDateTime;

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(list_transactions).post(create_transaction))
        .route(
            "/{id}",
            get(get_transaction)
                .put(replace_transaction)
                .patch(modify_transaction)
                .delete(delete_transaction),
        )
        .with_state(pool)
}

#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
    id: i32,
    description: String,
    amount: Decimal,
    category: String,
    timestamp: PrimitiveDateTime,
}
#[derive(Debug, Serialize, Deserialize)]
struct TransactionList {
    data: Vec<Transaction>,
}
async fn list_transactions(State(pool): State<PgPool>) -> Result<Json<TransactionList>> {
    Ok(Json(TransactionList {
        data: sqlx::query_as!(Transaction, "SELECT * FROM budget;")
            .fetch_all(&pool)
            .await?,
    }))
}

#[derive(Debug, Serialize, Deserialize)]
struct NewTransaction {
    description: String,
    amount: Decimal,
    category: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct TransactionId {
    id: i32,
}
async fn create_transaction(
    State(pool): State<PgPool>,
    Json(NewTransaction {
        description,
        amount,
        category,
    }): Json<NewTransaction>,
) -> Result<(StatusCode, [(HeaderName, i32); 1], Json<Transaction>)> {
    let transaction = sqlx::query_as!(
        Transaction,
        "INSERT INTO budget (description, amount, category) VALUES ($1, $2, $3) RETURNING *;",
        description,
        amount,
        category,
    )
    .fetch_one(&pool)
    .await?;
    Ok((
        StatusCode::CREATED,
        [(header::LOCATION, transaction.id)],
        Json(transaction),
    ))
}

async fn get_transaction(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<Response> {
    Ok(
        sqlx::query_as!(Transaction, "SELECT * FROM budget WHERE id = $1;", id)
            .fetch_optional(&pool)
            .await?
            .map(|transaction| Json(transaction).into_response())
            .unwrap_or(StatusCode::NOT_FOUND.into_response()),
    )
}

async fn delete_transaction(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<StatusCode> {
    Ok(sqlx::query_as!(
        TransactionId,
        "DELETE FROM budget WHERE id = $1 RETURNING id;",
        id
    )
    .fetch_optional(&pool)
    .await?
    .map(|_| StatusCode::NO_CONTENT)
    .unwrap_or(StatusCode::NOT_FOUND))
}

async fn replace_transaction(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(NewTransaction {
        description,
        amount,
        category,
    }): Json<NewTransaction>,
) -> Result<Response> {
    Ok(sqlx::query_as!(
        Transaction,
        "UPDATE budget SET description = $1, amount = $2, category = $3 WHERE id = $4 RETURNING *;",
        description,
        amount,
        category,
        id
    )
    .fetch_optional(&pool)
    .await?
    .map(|transaction| Json(transaction).into_response())
    .unwrap_or(StatusCode::NOT_FOUND.into_response()))
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateTransaction {
    description: Option<String>,
    amount: Option<Decimal>,
    category: Option<String>,
    update_timestamp: Option<bool>,
}

async fn modify_transaction(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(UpdateTransaction {
        description,
        amount,
        category,
        update_timestamp,
    }): Json<UpdateTransaction>,
) -> Result<Response> {
    // TODO: build the query dynamically with `sqlx::query`?
    if let Some(description) = description {
        sqlx::query!(
            "UPDATE budget SET description = $1 WHERE id = $2;",
            description,
            id
        )
        .execute(&pool)
        .await?;
    }
    if let Some(amount) = amount {
        sqlx::query!("UPDATE budget SET amount = $1 WHERE id = $2;", amount, id)
            .execute(&pool)
            .await?;
    }

    if let Some(category) = category {
        sqlx::query!(
            "UPDATE budget SET category = $1 WHERE id = $2;",
            category,
            id
        )
        .execute(&pool)
        .await?;
    }

    if let Some(update_timestamp) = update_timestamp
        && update_timestamp
    {
        sqlx::query!("UPDATE budget SET timestamp = DEFAULT WHERE id = $1;", id)
            .execute(&pool)
            .await?;
    }

    Ok(
        sqlx::query_as!(Transaction, "SELECT * FROM budget WHERE id = $1", id)
            .fetch_optional(&pool)
            .await?
            .map(|transaction| Json(transaction).into_response())
            .unwrap_or(StatusCode::NOT_FOUND.into_response()),
    )
}
