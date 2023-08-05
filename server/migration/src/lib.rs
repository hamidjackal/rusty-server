pub use sea_orm_migration::prelude::*;

mod m20230801_141102_create_user_table;
mod m20230802_091400_make_email_unique;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230801_141102_create_user_table::Migration),
            Box::new(m20230802_091400_make_email_unique::Migration),
        ]
    }
}
