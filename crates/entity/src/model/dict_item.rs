//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa :: ToSchema,
)]
#[sea_orm(table_name = "dict_item")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub dict_id: i64,
    pub key: String,
    pub name: String,
    pub value: String,
    pub sort: u32,
    pub remark: String,
    pub is_enabled: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
