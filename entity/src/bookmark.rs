//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "bookmark")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub url: String,
    pub description: Option<String>,
    pub title: Option<String>,
    pub creation_date: DateTimeWithTimeZone,
    pub update_date: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
