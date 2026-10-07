use super::MySqlRepository;
use crate::db::repo::{KvsRepository, PlanRepository, UserRepository};

/// CI supplies a real MySQL 5.7 service. Local test runs skip this integration
/// smoke test unless TEST_MYSQL_URL is explicitly set.
#[tokio::test]
async fn mysql_57_schema_and_repository_smoke() {
    let Ok(url) = std::env::var("TEST_MYSQL_URL") else {
        return;
    };
    let pool = crate::db::init::init_mysql(&url)
        .await
        .expect("initialize MySQL 5.7 schema");
    let repo = MySqlRepository::new(pool);

    let admin = repo
        .find_by_username("admin")
        .await
        .expect("query admin")
        .expect("seeded admin");
    assert!(admin.admin);
    assert!(!admin.password.contains("PLACEHOLDER"));

    let plans = repo.list_plans().await.expect("list plans");
    assert!(plans.iter().any(|plan| plan.name == "free"));

    repo.set("mysql:smoke", "ok").await.expect("upsert kv");
    assert_eq!(
        repo.get("mysql:smoke").await.expect("read kv").as_deref(),
        Some("ok")
    );
}
