extern crate core;

use sea_orm_migration::prelude::*;
use migration::prepare_connection;

#[async_std::main]
async fn main() {
    prepare_connection().await;
    cli::run_cli(migration::Migrator).await;
}

