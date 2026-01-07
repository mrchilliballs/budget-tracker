use std::env;

use axum::{Router, extract::State, http::StatusCode, routing::get};
use color_eyre::eyre::Result;
use rust_decimal::Decimal;
use sqlx::{PgPool, Row, migrate};
use tokio::net::TcpListener;

// mod old_main;

// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct Transaction {
//     // TODO: time
//     id:
//     kind: TransactionKind,
//     amount: Decimal,
//     category: String,
// }

#[tokio::main]
async fn main() -> Result<()> {
    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;
    migrate!().run(&pool).await?;

    let app = Router::new()
        .route("/summary", get(summary))
        .with_state(pool);

    let addr = "0.0.0.0:8000";
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on {}...", addr);
    axum::serve(listener, app).await?;
    Ok(())
}

// TODO: error handling, look at example
async fn summary(State(pool): State<PgPool>) -> StatusCode {
    println!("GET /summary");
    let rows = sqlx::query!(
        "
        SELECT description, amount
        FROM budget;
    ",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    // for (i, row) in rows.iter().enumerate() {
    //     println!("{i}: {:?}", row.columns());
    //     // let row: (String, Decimal) = row.into();
    // }

    println!("{rows:?}");

    StatusCode::OK
}
