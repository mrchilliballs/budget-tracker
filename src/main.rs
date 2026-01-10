use std::env;

use axum::Router;
use error::Result;
use sqlx::{PgPool, migrate};
use tokio::net::TcpListener;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

mod error;
mod summary;
mod transaction;

async fn app() -> Result<Router> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;
    migrate!().run(&pool).await?;

    Ok(Router::new()
        .nest("/transaction", transaction::router(pool.clone()))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
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
