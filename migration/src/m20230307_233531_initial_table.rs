use sea_orm_migration::prelude::*;
use crate::sea_orm::{ConnectionTrait, Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            CREATE TABLE IF NOT EXISTS ronin.transfers
            (
                id                varchar                        not null
                    constraint transfers_pk
                        primary key,
                token_type        varchar                        not null,
                block_number      integer                        not null,
                contract          varchar                        not null,
                "from"            varchar                        not null,
                "to"              varchar                        not null,
                value             varchar,
                token_id          varchar,
                transaction_hash  varchar                        not null,
                transaction_index varchar                        not null,
                log_index         varchar                        not null,
                ingested_at       timestamp with time zone default CURRENT_TIMESTAMP not null
            );
        "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE ronin.transfers;";
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
