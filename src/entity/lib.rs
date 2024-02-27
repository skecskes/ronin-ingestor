use std::env;
use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

pub async fn db_conn() -> anyhow::Result<DatabaseConnection, DbErr> {
    let username = &env::var("PGUSER").expect("Invalid postgres user in config");
    let password = &env::var("PGPASSWORD").expect("Invalid postgres password in config");
    let port = env::var("PGPORT").unwrap_or(String::from("5432")).parse::<u16>().expect("Invalid port specification");
    let host = &env::var("PGHOST").expect("Invalid postgres host in config");
    let database = &env::var("PGDATABASE").expect("Invalid postgres DB in config");
    let url = format!("postgres://{username}:{password}@{host}:{port}/{database}");

    let mut opt = ConnectOptions::new(url);
    opt.max_connections(4)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false)
        .sqlx_logging_level(log::LevelFilter::Warn)
        .set_schema_search_path::<String>("ronin".into()); // Setting default PostgreSQL schema

    Database::connect(opt).await
}