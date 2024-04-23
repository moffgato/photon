//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "token_accounts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub hash: Vec<u8>,
    pub owner: Vec<u8>,
    pub mint: Vec<u8>,
    pub delegate: Option<Vec<u8>>,
    pub state: i32,
    pub spent: bool,
    #[sea_orm(column_type = "Decimal(Some((20, 0)))")]
    pub amount: Decimal,
    #[sea_orm(column_type = "Decimal(Some((20, 0)))", nullable)]
    pub is_native: Option<Decimal>,
    #[sea_orm(column_type = "Decimal(Some((20, 0)))")]
    pub delegated_amount: Decimal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::accounts::Entity",
        from = "Column::Hash",
        to = "super::accounts::Column::Hash",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Accounts,
}

impl Related<super::accounts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Accounts.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
