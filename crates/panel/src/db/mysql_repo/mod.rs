// MySQL 5.7 implementation of the Repository traits.
//
// Handlers call the backend-neutral Repository traits; all production SQL in
// this module uses MySQL's `?` placeholders and MySQL 5.7-compatible syntax.

use sqlx::{Acquire, MySqlPool};

mod announcements;
mod groups;
mod kvs;
mod orders;
mod profiles;
mod redeem;
mod rules;
mod settings;
mod stats;
#[cfg(test)]
mod tests;
mod traffic;
mod user_groups;
mod users;

pub struct MySqlRepository {
    pub(super) pool: MySqlPool,
}

impl MySqlRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    /// Open an InnoDB transaction. Money and authorization paths use explicit
    /// `FOR UPDATE` reads where they must serialize concurrent changes.
    pub(super) async fn begin_write(
        &self,
    ) -> Result<sqlx::Transaction<'static, sqlx::MySql>, sqlx::Error> {
        self.pool.begin().await
    }
}

// ── Aggregate Repository ──
//
// The aggregate trait has no methods of its own — it just combines the domain
// traits. A blanket impl for any type satisfying all supertraits would work,
// but spelling it out keeps the impl block discoverable and avoids coherence
// surprises if another repository implementation is added later.
#[async_trait::async_trait]
impl super::repo::Repository for MySqlRepository {}
