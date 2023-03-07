use std::env;
pub use sea_orm_migration::prelude::*;
use crate::sea_orm::{ConnectionTrait, ConnectOptions, Database, DatabaseBackend, DatabaseConnection, Statement};

mod m20230307_233531_initial_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230307_233531_initial_table::Migration),
        ]
    }
}

pub async fn prepare_connection() -> DatabaseConnection {
    let url = env::var("DATABASE_URL").expect("Invalid postgres URL in config");
    let conn = create_db_conn(url).await;
    create_schemas(&conn).await;
    conn
}

pub async fn create_db_conn(url: String) -> DatabaseConnection {
    let opts = ConnectOptions::new(url)
        .set_schema_search_path("ronin".into())
        .to_owned();
    let db = Database::connect(opts).await;
    db.expect("Couldn't create DB connection")
}

pub async fn create_schemas(conn: &DatabaseConnection) {
    conn.execute(Statement::from_string(
        DatabaseBackend::Postgres,
        "CREATE SCHEMA IF NOT EXISTS ronin;".to_owned()))
        .await.expect("Couldn't create ronin schema");
}