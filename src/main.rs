use std::env;

use axum::Router;
use error::Result;
use sqlx::{PgPool, migrate};
use tokio::net::TcpListener;

mod error;
mod summary;
mod transaction;

async fn app() -> Result<Router> {
    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;
    migrate!().run(&pool).await?;

    Ok(Router::new()
        .nest("/transaction", transaction::router(pool.clone()))
        .merge(summary::router(pool.clone())))
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:8000";
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on {}...", addr);
    axum::serve(listener, app().await?).await?;
    Ok(())
}
