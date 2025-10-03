use sea_orm_migration::prelude::*;
use strum_macros::EnumIter;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20250925_000003_questions"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Question::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Question::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Question::Q).string())
                    .col(ColumnDef::new(Question::Opt1).string())
                    .col(ColumnDef::new(Question::Opt2).string())
                    .col(ColumnDef::new(Question::Opt3).string())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Bakery table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Question::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Question {
    Table,
    Id,
    Q,
    Opt1,
    Opt2,
    Opt3,
}

#[derive(Debug, Clone, PartialEq, Eq, sea_orm::DeriveActiveEnum, EnumIter)]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum Opt {
    #[sea_orm(string_value = "Student")]
    Student,
    #[sea_orm(string_value = "Teacher")]
    Teacher,
}
