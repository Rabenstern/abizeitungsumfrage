use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20250925_000004_answers"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Answer::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Answer::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Answer::Sid).integer().not_null())
                    .col(ColumnDef::new(Answer::Qid).integer().not_null())
                    .col(ColumnDef::new(Answer::Opt1).integer())
                    .col(ColumnDef::new(Answer::Opt2).integer())
                    .col(ColumnDef::new(Answer::Opt3).integer())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Bakery table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Answer::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Answer {
    Table,
    Id,
    Sid,
    Qid,
    Opt1,
    Opt2,
    Opt3,
}
