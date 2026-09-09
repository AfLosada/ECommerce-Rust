use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260909_005453_user_first"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table("user")
            .if_not_exists()
            .col(pk_auto("id"))
            .col(string("username"))
            .col(string("email"))
            .col(string("password"))
            .clone();
        _manager.create_table(table).await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        _manager
            .drop_table(Table::drop().table("user").clone())
            .await
    }
}
