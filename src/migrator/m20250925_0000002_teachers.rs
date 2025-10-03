use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20250925_000002_teachers"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Teacher::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Teacher::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Teacher::FirstName).string().not_null())
                    .col(ColumnDef::new(Teacher::LastName).string().not_null())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Bakery table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Teacher::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Teacher {
    Table,
    Id,
    FirstName,
    LastName,
}
