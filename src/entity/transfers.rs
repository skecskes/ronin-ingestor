//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.7

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "ronin", table_name = "transfers")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub token_type: String,
    pub block_number: i32,
    pub contract: String,
    pub from: String,
    pub to: String,
    pub value: Option<String>,
    pub token_id: Option<String>,
    pub transaction_hash: String,
    pub transaction_index: String,
    pub log_index: String,
    pub ingested_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}