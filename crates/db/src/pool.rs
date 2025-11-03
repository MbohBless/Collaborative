use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use anyhow::Result;

pub async fn create_pool(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    tracing::info!("✅ Database connection pool created");
    Ok(pool)
}