use sqlx::{SqlitePool, migrate::MigrateDatabase};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

pub struct Db {
    pub pool: SqlitePool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct QuantumBatch {
    pub id: i64,
    pub name: String,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct QuantumEntropyData {
    pub id: i64,
    pub batch_id: i64,
    pub pulse_round: Option<i64>,
    pub hex_value: String,
    pub created_at: Option<NaiveDateTime>,
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

    // === QUANTUM BATCH OPERATIONS ===

    pub async fn create_batch(&self, name: &str) -> Result<i64> {
        let id = sqlx::query("INSERT INTO quantum_entropy_batches (name, status) VALUES (?, 'collecting')")
            .bind(name)
            .execute(&self.pool)
            .await?
            .last_insert_rowid();
        Ok(id)
    }

    pub async fn get_batch(&self, id: i64) -> Result<QuantumBatch> {
        let batch = sqlx::query_as::<_, QuantumBatch>("SELECT * FROM quantum_entropy_batches WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(batch)
    }

    pub async fn list_batches(&self) -> Result<Vec<QuantumBatch>> {
        let batches = sqlx::query_as::<_, QuantumBatch>("SELECT * FROM quantum_entropy_batches ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;
        Ok(batches)
    }

    pub async fn update_batch_status(&self, id: i64, status: &str) -> Result<()> {
        sqlx::query("UPDATE quantum_entropy_batches SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(status)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn insert_entropy(&self, batch_id: i64, pulse_round: Option<u64>, hex_value: &str) -> Result<()> {
        sqlx::query("INSERT INTO quantum_entropy_data (batch_id, pulse_round, hex_value) VALUES (?, ?, ?)")
            .bind(batch_id)
            .bind(pulse_round.map(|v| v as i64))
            .bind(hex_value)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_batch_entropy(&self, batch_id: i64) -> Result<Vec<QuantumEntropyData>> {
        let data = sqlx::query_as::<_, QuantumEntropyData>("SELECT * FROM quantum_entropy_data WHERE batch_id = ? ORDER BY id ASC")
            .bind(batch_id)
            .fetch_all(&self.pool)
            .await?;
        Ok(data)
    }

    pub async fn get_batch_size(&self, batch_id: i64) -> Result<i64> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM quantum_entropy_data WHERE batch_id = ?")
            .bind(batch_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(row.0)
    }
}
