pub use sea_orm_migration::prelude::*;

mod m20260529_144919_create_settlemate_tables;
mod m20260804_000001_add_amount_cents;
mod m20260805_000001_create_sessions;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260529_144919_create_settlemate_tables::Migration),
            Box::new(m20260804_000001_add_amount_cents::Migration),
            Box::new(m20260805_000001_create_sessions::Migration),
        ]
    }
}
