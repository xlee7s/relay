//! Fresh-install schema for MySQL 5.7.
//!
//! The application stores timestamps as UTC text (`YYYY-MM-DD HH:MM:SS`) to
//! preserve the existing API/model contract. MySQL 5.7 cannot use an
//! expression default on a CHAR column, so small BEFORE INSERT triggers fill
//! `created_at` when repository inserts omit it.

pub const MYSQL_SCHEMA_VERSION: i32 = 1;

pub const MYSQL_SCHEMA_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS plans (
    id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    max_rules INT NOT NULL DEFAULT 5,
    traffic BIGINT NOT NULL DEFAULT 0,
    speed_limit INT NOT NULL DEFAULT 0,
    ip_limit INT NOT NULL DEFAULT 3,
    price VARCHAR(64) NOT NULL DEFAULT '0',
    plan_type VARCHAR(32) NOT NULL DEFAULT 'data',
    duration_days INT NOT NULL DEFAULT 0,
    hidden TINYINT(1) NOT NULL DEFAULT 0,
    reset_traffic TINYINT(1) NOT NULL DEFAULT 0,
    description TEXT NOT NULL,
    grant_all_groups TINYINT(1) NOT NULL DEFAULT 0,
    created_at CHAR(19) NOT NULL DEFAULT ''
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS users (
    id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(191) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    balance VARCHAR(64) NOT NULL DEFAULT '0',
    plan_id BIGINT NULL,
    all_device_groups TINYINT(1) NOT NULL DEFAULT 0,
    max_rules INT NOT NULL DEFAULT 5,
    speed_limit INT NOT NULL DEFAULT 0,
    ip_limit INT NOT NULL DEFAULT 3,
    traffic_used BIGINT NOT NULL DEFAULT 0,
    traffic_limit BIGINT NOT NULL DEFAULT 0,
    admin TINYINT(1) NOT NULL DEFAULT 0,
    banned TINYINT(1) NOT NULL DEFAULT 0,
    created_at CHAR(19) NOT NULL DEFAULT '',
    must_change_password TINYINT(1) NOT NULL DEFAULT 0,
    token_version BIGINT NOT NULL DEFAULT 0,
    plan_expire_at CHAR(19) NULL,
    suspended TINYINT(1) NOT NULL DEFAULT 0,
    CONSTRAINT fk_users_plan FOREIGN KEY (plan_id) REFERENCES plans(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS device_groups (
    id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    group_type VARCHAR(32) NOT NULL,
    token VARCHAR(191) NOT NULL UNIQUE,
    uid BIGINT NOT NULL,
    connect_host VARCHAR(255) NOT NULL DEFAULT '',
    port_range VARCHAR(255) NOT NULL DEFAULT '1-65535',
    fallback_group BIGINT NULL,
    config VARCHAR(4096) NOT NULL DEFAULT '{}',
    capabilities VARCHAR(255) NOT NULL DEFAULT '[\"tcp\",\"udp\"]',
    region VARCHAR(255) NULL,
    line_type VARCHAR(255) NULL,
    remark TEXT NULL,
    rate DOUBLE NOT NULL DEFAULT 1.0,
    hidden TINYINT(1) NOT NULL DEFAULT 0,
    created_at CHAR(19) NOT NULL DEFAULT '',
    CONSTRAINT fk_device_groups_user FOREIGN KEY (uid) REFERENCES users(id),
    CONSTRAINT fk_device_groups_fallback FOREIGN KEY (fallback_group) REFERENCES device_groups(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS tunnel_profiles (
    id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(191) NOT NULL UNIQUE,
    transport VARCHAR(32) NOT NULL DEFAULT 'direct',
    tls_mode VARCHAR(32) NOT NULL DEFAULT 'none',
    ws_path VARCHAR(255) NOT NULL DEFAULT '/relay',
    host_header VARCHAR(255) NOT NULL DEFAULT '',
    sni VARCHAR(255) NOT NULL DEFAULT '',
    cert_id BIGINT NULL,
    is_builtin TINYINT(1) NOT NULL DEFAULT 0,
    uid BIGINT NOT NULL,
    created_at CHAR(19) NOT NULL DEFAULT '',
    CONSTRAINT fk_tunnel_profiles_user FOREIGN KEY (uid) REFERENCES users(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS forward_rules (
    id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    uid BIGINT NOT NULL,
    paused TINYINT(1) NOT NULL DEFAULT 0,
    auto_paused TINYINT(1) NOT NULL DEFAULT 0,
    listen_port INT NOT NULL,
    protocol VARCHAR(32) NOT NULL DEFAULT 'tcp',
    public_transport VARCHAR(32) NOT NULL DEFAULT 'raw',
    node_transport VARCHAR(32) NOT NULL DEFAULT 'raw',
    route_mode VARCHAR(32) NOT NULL DEFAULT 'direct',
    entry_transport VARCHAR(32) NOT NULL DEFAULT 'raw',
    device_group_in BIGINT NOT NULL,
    device_group_out BIGINT NULL,
    forward_mode VARCHAR(32) NOT NULL DEFAULT 'group',
    tunnel_profile_id BIGINT NULL,
    domain VARCHAR(255) NULL,
    ws_path VARCHAR(255) NULL,
    ws_host VARCHAR(255) NULL,
    sni VARCHAR(255) NULL,
    target_addr VARCHAR(255) NOT NULL,
    target_port INT NOT NULL,
    load_balance_strategy VARCHAR(32) NOT NULL DEFAULT 'first',
    upload_limit_mbps INT NOT NULL DEFAULT 0,
    download_limit_mbps INT NOT NULL DEFAULT 0,
    max_connections INT NOT NULL DEFAULT 0,
    auto_restart_minutes INT NOT NULL DEFAULT 0,
    config VARCHAR(4096) NOT NULL DEFAULT '{}',
    traffic_used BIGINT NOT NULL DEFAULT 0,
    status VARCHAR(32) NOT NULL DEFAULT 'active',
    created_at CHAR(19) NOT NULL DEFAULT '',
    tcp_listen_port INT GENERATED ALWAYS AS
        (CASE WHEN protocol IN ('tcp','tcp_udp') THEN listen_port ELSE NULL END) STORED,
    udp_listen_port INT GENERATED ALWAYS AS
        (CASE WHEN protocol IN ('udp','tcp_udp') THEN listen_port ELSE NULL END) STORED,
    UNIQUE KEY uq_forward_rules_tcp (device_group_in, tcp_listen_port),
    UNIQUE KEY uq_forward_rules_udp (device_group_in, udp_listen_port),
    KEY idx_forward_rules_uid (uid),
    KEY idx_forward_rules_profile (tunnel_profile_id),
    CONSTRAINT fk_forward_rules_user FOREIGN KEY (uid) REFERENCES users(id),
    CONSTRAINT fk_forward_rules_group_in FOREIGN KEY (device_group_in) REFERENCES device_groups(id),
    CONSTRAINT fk_forward_rules_group_out FOREIGN KEY (device_group_out) REFERENCES device_groups(id),
    CONSTRAINT fk_forward_rules_profile FOREIGN KEY (tunnel_profile_id) REFERENCES tunnel_profiles(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS forward_rule_targets (
    id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    rule_id BIGINT NOT NULL,
    host VARCHAR(255) NOT NULL,
    port INT NOT NULL,
    position INT NOT NULL,
    enabled TINYINT(1) NOT NULL DEFAULT 1,
    created_at CHAR(19) NOT NULL DEFAULT '',
    KEY idx_forward_rule_targets_rule_position (rule_id, position),
    CONSTRAINT fk_forward_rule_targets_rule FOREIGN KEY (rule_id) REFERENCES forward_rules(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS statistics (
    id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    stat_type VARCHAR(191) NOT NULL,
    stat_key VARCHAR(191) NOT NULL,
    time VARCHAR(64) NOT NULL,
    number BIGINT NOT NULL DEFAULT 0
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS orders (
    id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    user_id BIGINT NOT NULL,
    plan_id BIGINT NULL,
    plan_name VARCHAR(255) NOT NULL,
    price VARCHAR(64) NOT NULL,
    created_at CHAR(19) NOT NULL DEFAULT '',
    KEY idx_orders_user_id (user_id),
    CONSTRAINT fk_orders_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS redeem_codes (
    id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    code VARCHAR(191) NOT NULL UNIQUE,
    amount VARCHAR(64) NOT NULL,
    status VARCHAR(32) NOT NULL DEFAULT 'unused',
    used_by BIGINT NULL,
    used_at CHAR(19) NULL,
    expires_at CHAR(19) NULL,
    batch_id VARCHAR(191) NOT NULL DEFAULT '',
    remark TEXT NOT NULL,
    created_at CHAR(19) NOT NULL DEFAULT '',
    KEY idx_redeem_codes_status (status),
    KEY idx_redeem_codes_batch (batch_id),
    KEY idx_redeem_codes_used_by (used_by),
    CONSTRAINT fk_redeem_codes_user FOREIGN KEY (used_by) REFERENCES users(id) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS traffic_history (
    rule_id BIGINT NOT NULL,
    uid BIGINT NOT NULL,
    group_id BIGINT NOT NULL DEFAULT 0,
    hour_ts CHAR(19) NOT NULL,
    real_upload BIGINT NOT NULL DEFAULT 0,
    real_download BIGINT NOT NULL DEFAULT 0,
    billed_total BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (rule_id, hour_ts),
    KEY idx_traffic_history_uid (uid, hour_ts),
    KEY idx_traffic_history_hour (hour_ts)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS node_metrics_history (
    node_id VARCHAR(191) NOT NULL,
    group_id BIGINT NOT NULL DEFAULT 0,
    hour_ts CHAR(19) NOT NULL,
    samples BIGINT NOT NULL DEFAULT 0,
    cpu_sum DOUBLE NOT NULL DEFAULT 0,
    cpu_max DOUBLE NOT NULL DEFAULT 0,
    mem_sum DOUBLE NOT NULL DEFAULT 0,
    mem_max DOUBLE NOT NULL DEFAULT 0,
    conn_sum BIGINT NOT NULL DEFAULT 0,
    conn_max BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (node_id, hour_ts),
    KEY idx_node_metrics_hour (hour_ts),
    KEY idx_node_metrics_group (group_id, hour_ts)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS audit_log (
    id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    ts CHAR(19) NOT NULL,
    actor_id BIGINT NULL,
    actor_name VARCHAR(255) NOT NULL DEFAULT '',
    action VARCHAR(191) NOT NULL,
    target_type VARCHAR(191) NOT NULL DEFAULT '',
    target_id VARCHAR(191) NOT NULL DEFAULT '',
    detail TEXT NOT NULL,
    KEY idx_audit_log_ts (ts),
    KEY idx_audit_log_action (action, ts)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS announcements (
    id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(255) NOT NULL DEFAULT '',
    content TEXT NOT NULL,
    kind VARCHAR(32) NOT NULL DEFAULT 'info',
    pinned TINYINT(1) NOT NULL DEFAULT 0,
    published_at CHAR(19) NOT NULL,
    expires_at CHAR(19) NULL,
    author_id BIGINT NULL,
    author_name VARCHAR(255) NOT NULL DEFAULT '',
    KEY idx_announcements_published (published_at),
    KEY idx_announcements_pinned (pinned, published_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS traffic_batches (
    group_id BIGINT NOT NULL,
    batch_id VARCHAR(191) NOT NULL,
    created_at CHAR(19) NOT NULL DEFAULT '',
    confirmed_at CHAR(19) NULL,
    PRIMARY KEY (group_id, batch_id),
    KEY idx_traffic_batches_created (created_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS plan_device_groups (
    plan_id BIGINT NOT NULL,
    device_group_id BIGINT NOT NULL,
    PRIMARY KEY (plan_id, device_group_id),
    CONSTRAINT fk_plan_device_groups_plan FOREIGN KEY (plan_id) REFERENCES plans(id) ON DELETE CASCADE,
    CONSTRAINT fk_plan_device_groups_group FOREIGN KEY (device_group_id) REFERENCES device_groups(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS kvs (
    `key` VARCHAR(191) NOT NULL PRIMARY KEY,
    `value` LONGTEXT NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS app_settings (
    id INT NOT NULL PRIMARY KEY,
    registration_enabled TINYINT(1) NOT NULL DEFAULT 0,
    default_registration_plan_id BIGINT NOT NULL DEFAULT 1,
    registration_allowed_plan_ids VARCHAR(4096) NOT NULL DEFAULT '[1]',
    CONSTRAINT fk_app_settings_plan FOREIGN KEY (default_registration_plan_id) REFERENCES plans(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS user_device_groups (
    user_id BIGINT NOT NULL,
    device_group_id BIGINT NOT NULL,
    PRIMARY KEY (user_id, device_group_id),
    CONSTRAINT fk_user_device_groups_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_user_device_groups_group FOREIGN KEY (device_group_id) REFERENCES device_groups(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS schema_version (
    version INT NOT NULL PRIMARY KEY,
    applied_at CHAR(19) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

DROP TRIGGER IF EXISTS plans_created_at;
CREATE TRIGGER plans_created_at BEFORE INSERT ON plans FOR EACH ROW SET NEW.created_at = COALESCE(NULLIF(NEW.created_at, ''), DATE_FORMAT(UTC_TIMESTAMP(), '%Y-%m-%d %H:%i:%s'));
DROP TRIGGER IF EXISTS users_created_at;
CREATE TRIGGER users_created_at BEFORE INSERT ON users FOR EACH ROW SET NEW.created_at = COALESCE(NULLIF(NEW.created_at, ''), DATE_FORMAT(UTC_TIMESTAMP(), '%Y-%m-%d %H:%i:%s'));
DROP TRIGGER IF EXISTS device_groups_created_at;
CREATE TRIGGER device_groups_created_at BEFORE INSERT ON device_groups FOR EACH ROW SET NEW.created_at = COALESCE(NULLIF(NEW.created_at, ''), DATE_FORMAT(UTC_TIMESTAMP(), '%Y-%m-%d %H:%i:%s'));
DROP TRIGGER IF EXISTS tunnel_profiles_created_at;
CREATE TRIGGER tunnel_profiles_created_at BEFORE INSERT ON tunnel_profiles FOR EACH ROW SET NEW.created_at = COALESCE(NULLIF(NEW.created_at, ''), DATE_FORMAT(UTC_TIMESTAMP(), '%Y-%m-%d %H:%i:%s'));
DROP TRIGGER IF EXISTS forward_rules_created_at;
CREATE TRIGGER forward_rules_created_at BEFORE INSERT ON forward_rules FOR EACH ROW SET NEW.created_at = COALESCE(NULLIF(NEW.created_at, ''), DATE_FORMAT(UTC_TIMESTAMP(), '%Y-%m-%d %H:%i:%s'));
DROP TRIGGER IF EXISTS forward_rule_targets_created_at;
CREATE TRIGGER forward_rule_targets_created_at BEFORE INSERT ON forward_rule_targets FOR EACH ROW SET NEW.created_at = COALESCE(NULLIF(NEW.created_at, ''), DATE_FORMAT(UTC_TIMESTAMP(), '%Y-%m-%d %H:%i:%s'));
DROP TRIGGER IF EXISTS orders_created_at;
CREATE TRIGGER orders_created_at BEFORE INSERT ON orders FOR EACH ROW SET NEW.created_at = COALESCE(NULLIF(NEW.created_at, ''), DATE_FORMAT(UTC_TIMESTAMP(), '%Y-%m-%d %H:%i:%s'));
DROP TRIGGER IF EXISTS redeem_codes_created_at;
CREATE TRIGGER redeem_codes_created_at BEFORE INSERT ON redeem_codes FOR EACH ROW SET NEW.created_at = COALESCE(NULLIF(NEW.created_at, ''), DATE_FORMAT(UTC_TIMESTAMP(), '%Y-%m-%d %H:%i:%s'));

INSERT IGNORE INTO plans (id, name, max_rules, traffic, speed_limit, ip_limit, price, description, created_at)
VALUES (1, 'free', 5, 107374182400, 0, 3, '0', '', '');
INSERT IGNORE INTO users (id, username, password, admin, max_rules, created_at)
VALUES (1, 'admin', '$2b$12$PLACEHOLDER_WILL_BE_HASHED_ON_INIT', 1, 999, '');
INSERT IGNORE INTO tunnel_profiles (name, transport, tls_mode, ws_path, host_header, sni, is_builtin, uid, created_at) VALUES
    ('direct', 'direct', 'none', '', '', '', 1, 1, ''),
    ('ws-relay', 'ws', 'none', '/relay', '', '', 1, 1, ''),
    ('tls-passthrough', 'tls', 'passthrough', '', '', '', 1, 1, ''),
    ('tls-terminate', 'tls', 'terminate', '', '', '', 1, 1, '');
INSERT IGNORE INTO schema_version (version, applied_at)
VALUES (1, DATE_FORMAT(UTC_TIMESTAMP(), '%Y-%m-%d %H:%i:%s'));
"#;

pub async fn apply_mysql_schema(pool: &sqlx::MySqlPool) -> Result<(), sqlx::Error> {
    let exists: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM information_schema.tables \
         WHERE table_schema = DATABASE() AND table_name = 'schema_version'",
    )
    .fetch_one(pool)
    .await?;

    if exists == 0 {
        for statement in MYSQL_SCHEMA_SQL.split(';').map(str::trim).filter(|s| !s.is_empty()) {
            sqlx::query(statement).execute(pool).await?;
        }
    }

    let version: i32 = sqlx::query_scalar("SELECT COALESCE(MAX(version), 0) FROM schema_version")
        .fetch_one(pool)
        .await?;
    if version != MYSQL_SCHEMA_VERSION {
        return Err(sqlx::Error::Protocol(format!(
            "unsupported MySQL schema version {version}; expected {MYSQL_SCHEMA_VERSION}"
        )));
    }
    Ok(())
}
