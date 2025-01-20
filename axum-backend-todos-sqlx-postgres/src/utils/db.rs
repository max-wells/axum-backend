use sqlx::PgPool;

pub type Db = PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
}

pub async fn create_db_pool() -> sqlx::Result<PgPool> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
