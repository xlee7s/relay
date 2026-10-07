// v0.4.3: Unified database error type.
//
// Hides backend-specific constraint codes behind a single enum. Production
// uses MySQL; SQLite codes remain for the isolated unit-test repository.
// instead of raw error codes, so the same handler code works on both backends.
//
// The `Other` variant retains the underlying sqlx::Error for logging (via
// tracing::error!), but handlers MUST NOT return its stringified form to the
// API client — use a generic message instead (e.g. "database error") to avoid
// leaking schema/SQL details.

/// A unified repository error. Every Repository method returns
/// `Result<T, DbError>`.
#[derive(Debug)]
pub enum DbError {
    /// UNIQUE constraint violation (MySQL 1062; SQLite test fixture 2067).
    UniqueViolation,
    /// v0.4.11 PR4: a listen_port is already occupied on the rule's inbound
    /// group by a conflicting socket type (TCP vs UDP). Distinct from
    /// `UniqueViolation` so handlers can return a clear, port-specific 409.
    /// Detected by the in-transaction conflict pre-check; the partial unique
    /// indexes on forward_rules are the DB-layer backstop.
    PortConflict,
    /// A state transition would exceed the owner's active-rule allowance.
    /// This is distinct from creation returning 0 rows: callers need to tell
    /// an existing paused rule from a missing rule when a resume is rejected.
    QuotaExceeded,
    /// v1.2.12: the write would put a rule on an inbound group its owner is
    /// not authorized for — re-checked inside the write transaction, so an
    /// authorization revoked after the API's own check still counts.
    GroupNotAuthorized,
    /// FOREIGN KEY constraint violation (MySQL 1451/1452; SQLite fixture 787).
    ForeignKeyViolation,
    /// A required row was not found (for fetch_one-or-None patterns that are
    /// expected to succeed).
    NotFound,
    /// Any other database error. The inner sqlx::Error is retained for
    /// logging but should NOT be serialized into an API response.
    Other(sqlx::Error),
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::UniqueViolation => write!(f, "unique constraint violation"),
            DbError::PortConflict => write!(f, "listen_port conflict on inbound group"),
            DbError::QuotaExceeded => write!(f, "active rule quota exceeded"),
            DbError::GroupNotAuthorized => {
                write!(f, "device group not authorized for the rule's owner")
            }
            DbError::ForeignKeyViolation => write!(f, "foreign key constraint violation"),
            DbError::NotFound => write!(f, "not found"),
            DbError::Other(e) => write!(f, "database error: {}", e),
        }
    }
}

impl std::error::Error for DbError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DbError::Other(e) => Some(e),
            _ => None,
        }
    }
}

impl From<sqlx::Error> for DbError {
    /// Map a raw sqlx::Error to a DbError by inspecting the database error code.
    fn from(e: sqlx::Error) -> Self {
        if let sqlx::Error::Database(db_err) = &e {
            match db_err.code().as_deref() {
                // SQLite SQLITE_CONSTRAINT_UNIQUE
                Some("2067") => return DbError::UniqueViolation,
                // MySQL ER_DUP_ENTRY
                Some("1062") => return DbError::UniqueViolation,
                // SQLite SQLITE_CONSTRAINT_FOREIGNKEY
                Some("787") => return DbError::ForeignKeyViolation,
                // MySQL ER_ROW_IS_REFERENCED_2 / ER_NO_REFERENCED_ROW_2
                Some("1451") | Some("1452") => return DbError::ForeignKeyViolation,
                _ => {}
            }
        }
        // RowNotFound → NotFound
        if matches!(e, sqlx::Error::RowNotFound) {
            return DbError::NotFound;
        }
        DbError::Other(e)
    }
}
