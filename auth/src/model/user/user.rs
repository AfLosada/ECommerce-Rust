use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user", rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: i32,
    first_name: String, // firstName
    #[sea_orm(column_name = "lAsTnAmE")]
    last_name: String, // lAsTnAmE
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
