use sqlx::{SqlitePool, migrate::MigrateDatabase};
use anyhow::Result;

pub struct Db {
    pub pool: SqlitePool,
}

impl Db {
    pub async fn new(db_url: &str) -> Result<Self> {
        if !sqlx::Sqlite::database_exists(db_url).await.unwrap_or(false) {
            println!("Creating database: {}", db_url);
            sqlx::Sqlite::create_database(db_url).await?;
        }

        let pool = SqlitePool::connect(db_url).await?;

        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await?;

        Ok(Self { pool })
    }
}
