use super::MySqlRepository;
use crate::db::error::DbError;
use crate::db::repo::*;
use async_trait::async_trait;

// ── KvsRepository ──

#[async_trait]
impl KvsRepository for MySqlRepository {
    async fn get(&self, key: &str) -> Result<Option<String>, DbError> {
        let row: Option<(String,)> = sqlx::query_as("SELECT `value` FROM kvs WHERE `key` = ?")
            .bind(key)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|(v,)| v))
    }

    async fn set(&self, key: &str, value: &str) -> Result<(), DbError> {
        sqlx::query(
            "INSERT INTO kvs (`key`, `value`) VALUES (?, ?) \
             ON DUPLICATE KEY UPDATE `value` = VALUES(`value`)",
        )
            .bind(key)
            .bind(value)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<u64, DbError> {
        let result = sqlx::query("DELETE FROM kvs WHERE `key` = ?")
            .bind(key)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }

    async fn scan_prefix(&self, prefix: &str) -> Result<Vec<(String, String)>, DbError> {
        // Current callers pass fixed prefixes such as `node_status:` (without
        // LIKE metacharacters), so appending `%` is a safe prefix scan.
        let pattern = format!("{}%", prefix);
        let rows: Vec<(String, String)> =
            sqlx::query_as("SELECT `key`, `value` FROM kvs WHERE `key` LIKE ?")
                .bind(&pattern)
                .fetch_all(&self.pool)
                .await?;
        Ok(rows)
    }
}
