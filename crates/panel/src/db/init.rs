use crate::db::repo::UserRepository;
use crate::db::schema::{run_migrations, SCHEMA_SQL};
use crate::db::sqlite_repo::SqliteRepository;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;

pub async fn init_db(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    // Parse the connection string and force three critical PRAGMAs that
    // prevent the most common SQLite pain points under concurrent load:
    //
    //   journal_mode=WAL  — allows concurrent readers + 1 writer without
    //     "database is locked" on multi-node traffic reporting.
    //   busy_timeout=5000 — if a write lock IS contended, wait up to 5s
    //     before erroring instead of failing instantly.
    //   foreign_keys=ON   — enques FK constraints (currently worked around
    //     by manual cascade in delete_user, but this catches future leaks).
    //
    // NOTE: foreign_keys is intentionally NOT set via SqliteConnectOptions here.
    // That pragma would be re-applied on every NEW pool connection, overriding
    // the temporary OFF we need during run_migrations (SQLite forbids dropping
    // a table referenced by an FK while enforcement is on, and several
    // migrations rebuild tables). Instead we toggle it explicitly below.
    //
    // SqliteConnectOptions::pragma() appends the others as `PRAGMA key=value;`
    // on every new connection, so they apply pool-wide.
    let opts = SqliteConnectOptions::from_str(database_url)?
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .busy_timeout(std::time::Duration::from_secs(5))
        .create_if_missing(true);

    // Single-connection pool for migration: with max_connections=1 we are
    // guaranteed that the PRAGMA foreign_keys=OFF we set on it stays in effect
    // for every migration statement (no second connection reseting it to ON).
    let migrate_pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(opts.clone())
        .await?;
    sqlx::query(SCHEMA_SQL).execute(&migrate_pool).await?;

    // foreign_keys OFF during schema maintenance, per SQLite guidance:
    // https://www.sqlite.org/foreignkeys.html#fk_schemabugs
    sqlx::query("PRAGMA foreign_keys = OFF")
        .execute(&migrate_pool)
        .await?;
    run_migrations(&migrate_pool).await?;
    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&migrate_pool)
        .await?;
    migrate_pool.close().await;

    // Now build the real serving pool with max_connections=5. FK enforcement
    // is turned on per-connection via the connect_options pragma so it always
    // holds for runtime queries.
    let opts = opts.foreign_keys(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(opts)
        .await?;

    // Replace the SCHEMA_SQL placeholder password with a real bcrypt hash of
    // the default "admin123" — but ONLY when the stored password is still the
    // placeholder. See hash_default_admin_password_if_placeholder for the
    // safety reasoning (the short version: the WHERE clause matches ONLY the
    // literal placeholder, never an admin-changed bcrypt hash).
    hash_default_admin_password_if_placeholder(&pool).await?;

    Ok(pool)
}
/// Hash the default "admin123" password into users(id=1) IF AND ONLY IF the
/// stored value is still the SCHEMA_SQL placeholder. A password an admin has
/// already changed is NEVER touched.
///
/// Safety (this is the v0.3.1 audit item #3 regression guard):
///   - `WHERE password LIKE '$2b$12$PLACEHOLDER%'` matches only the seeded
///     placeholder string, never a real bcrypt hash (real hashes have random
///     salt where the literal "PLACEHOLDER" sits).
///   - The expensive bcrypt hash is computed lazily inside the branch, so a
///     normal restart (password already set) does NOT pay ~100ms on every boot.
///   - Migrations 1-16 never touch the password column; INSERT OR IGNORE in
///     SCHEMA_SQL only inserts when the row is absent. So no code path resets
///     a changed password — confirmed by `password_survives_init_when_changed`.
pub async fn hash_default_admin_password_if_placeholder(
    pool: &sqlx::SqlitePool,
) -> Result<(), sqlx::Error> {
    // v0.4.3: route through the UserRepository trait so the placeholder-hash
    // SQL lives in exactly one place (sqlite_repo.rs), and PR2's PgRepository
    // gets the same guard for free. init.rs still owns the SqlitePool because
    // migrations are SQLite-specific (PRAGMA toggles, schema rebuilds) and can't
    // be expressed through the Repository abstraction.
    let repo = SqliteRepository::new(pool.clone());
    let needs_hash = repo
        .count_placeholder_admin_password()
        .await
        .map_err(db_err_to_sqlx)?;
    if needs_hash > 0 {
        let hashed = bcrypt::hash("admin123", 12).unwrap_or_default();
        repo.replace_placeholder_admin_password(&hashed)
            .await
            .map_err(db_err_to_sqlx)?;
        tracing::info!("hashed the default admin password (first boot placeholder -> bcrypt)");
    }
    Ok(())
}

/// Map a DbError back to sqlx::Error so init_db keeps its sqlx::Error return
/// type (callers expect it). The Other variant unwraps the wrapped sqlx::Error;
/// the structured variants (UniqueViolation etc.) are surfaced as generic
/// database errors — they cannot happen here, but if they did the right thing
/// is to fail init loudly.
fn db_err_to_sqlx(e: crate::db::error::DbError) -> sqlx::Error {
    match e {
        crate::db::error::DbError::Other(inner) => inner,
        other => sqlx::Error::Configuration(format!("init db error: {other}").into()),
    }
}
