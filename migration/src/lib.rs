pub use sea_orm_migration::prelude::*;

mod m20260529_144919_create_settlemate_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260529_144919_create_settlemate_tables::Migration),
        ]
    }
}
