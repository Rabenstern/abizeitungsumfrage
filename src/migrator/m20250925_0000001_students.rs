use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20250925_000001_students"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Student::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Student::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Student::Email).string().not_null())
                    .col(ColumnDef::new(Student::Token).string().not_null())
                    .col(ColumnDef::new(Student::FirstName).string().not_null())
                    .col(ColumnDef::new(Student::LastName).string().not_null())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Bakery table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Student::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Student {
    Table,
    Id,
    Email,
    Token,
    FirstName,
    LastName,
}
