//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa :: ToSchema,
)]
#[sea_orm(table_name = "sys_org")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub pid: i64,
    pub tier: String,
    pub name: String,
    pub leader: String,
    pub phone: String,
    pub email: String,
    pub sort: u32,
    pub remark: String,
    pub is_enabled: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
