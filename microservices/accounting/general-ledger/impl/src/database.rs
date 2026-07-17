//! Process-wide Lifeguard pool for the General Ledger runtime.

use std::sync::{Arc, OnceLock};

use lifeguard::{DatabaseConfig, LifeguardPool, PooledLifeExecutor};

static EXECUTOR: OnceLock<Result<PooledLifeExecutor, String>> = OnceLock::new();

fn non_empty_env(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .filter(|value| !value.trim().is_empty())
}

fn build_executor() -> Result<PooledLifeExecutor, String> {
    let host = std::env::var("DB_HOST").unwrap_or_else(|_| {
        std::env::var("KUBERNETES_SERVICE_HOST")
            .map(|_| "postgres.data.svc.cluster.local".to_string())
            .unwrap_or_else(|_| "localhost".to_string())
    });
    let port = std::env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
    let user = std::env::var("DB_USER").unwrap_or_else(|_| "rerp".to_string());
    let database = std::env::var("DB_NAME").unwrap_or_else(|_| "rerp".to_string());
    let password = non_empty_env("DB_PASS")
        .or_else(|| non_empty_env("RERP_DB_PASSWORD"))
        .unwrap_or_default();

    let mut url = format!("host={host} port={port} user={user} dbname={database}");
    if !password.is_empty() {
        url.push_str(&format!(" password={password}"));
    }
    let config = DatabaseConfig {
        url,
        max_connections: std::env::var("DB_POOL_MAX")
            .ok()
            .and_then(|value| value.parse().ok())
            .filter(|value| *value >= 1)
            .unwrap_or(2),
        ..Default::default()
    };
    let pool = LifeguardPool::from_database_config(&config, Vec::new(), 0)
        .map_err(|error| format!("database pool initialization failed: {error}"))?;
    println!(
        "[startup] Lifeguard pool ready for postgresql://{user}@{host}:{port}/{database} (primary slots={})",
        pool.primary_pool_size()
    );
    Ok(PooledLifeExecutor::new(Arc::new(pool)))
}

pub fn initialize() -> Result<(), String> {
    db().map(|_| ())
}

pub fn db() -> Result<&'static PooledLifeExecutor, String> {
    EXECUTOR
        .get_or_init(build_executor)
        .as_ref()
        .map_err(Clone::clone)
}
