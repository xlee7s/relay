use crate::db::mysql_repo::MySqlRepository;
use crate::db::mysql_schema::apply_mysql_schema;
use crate::db::repo::UserRepository;

/// Connect to MySQL 5.7, initialize a fresh schema when necessary, and fail
/// fast if MySQL is unavailable. There is deliberately no backend fallback.
pub async fn init_mysql(database_url: &str) -> Result<sqlx::MySqlPool, sqlx::Error> {
    use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
    use std::str::FromStr;

    if !is_mysql_url(database_url) {
        return Err(sqlx::Error::Configuration(
            "DATABASE_URL must start with mysql://".into(),
        ));
    }

    let options = MySqlConnectOptions::from_str(database_url)?;
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect_with(options)
        .await?;
    apply_mysql_schema(&pool).await?;
    hash_default_admin_password_if_placeholder(&pool).await?;
    Ok(pool)
}

async fn hash_default_admin_password_if_placeholder(
    pool: &sqlx::MySqlPool,
) -> Result<(), sqlx::Error> {
    let repo = MySqlRepository::new(pool.clone());
    let needs_hash = repo
        .count_placeholder_admin_password()
        .await
        .map_err(db_err_to_sqlx)?;
    if needs_hash > 0 {
        let hashed = bcrypt::hash("admin123", 12).unwrap_or_default();
        repo.replace_placeholder_admin_password(&hashed)
            .await
            .map_err(db_err_to_sqlx)?;
        tracing::info!("hashed the default admin password on first boot");
    }
    Ok(())
}

fn db_err_to_sqlx(e: crate::db::error::DbError) -> sqlx::Error {
    match e {
        crate::db::error::DbError::Other(inner) => inner,
        other => sqlx::Error::Configuration(format!("init db error: {other}").into()),
    }
}

pub fn is_mysql_url(database_url: &str) -> bool {
    database_url.starts_with("mysql://")
}

// SQLite remains available only to the existing isolated unit-test fixtures.
// It is not reachable from the production binary or deployment configuration.
#[cfg(test)]
pub async fn init_db(database_url: &str) -> Result<sqlx::SqlitePool, sqlx::Error> {
    use crate::db::schema::{run_migrations, SCHEMA_SQL};
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;

    let options = SqliteConnectOptions::from_str(database_url)?
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .busy_timeout(std::time::Duration::from_secs(5))
        .create_if_missing(true);
    let migrate_pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options.clone())
        .await?;
    sqlx::query(SCHEMA_SQL).execute(&migrate_pool).await?;
    sqlx::query("PRAGMA foreign_keys = OFF")
        .execute(&migrate_pool)
        .await?;
    run_migrations(&migrate_pool).await?;
    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&migrate_pool)
        .await?;
    migrate_pool.close().await;

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options.foreign_keys(true))
        .await
}
