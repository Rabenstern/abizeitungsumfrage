use sea_orm_migration::prelude::*;

mod m20250925_0000001_students;
mod m20250925_0000002_teachers;
mod m20250925_0000003_questions;
mod m20250925_0000004_answers;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250925_0000001_students::Migration),
            Box::new(m20250925_0000002_teachers::Migration),
            Box::new(m20250925_0000003_questions::Migration),
            Box::new(m20250925_0000004_answers::Migration),
        ]
    }
}
