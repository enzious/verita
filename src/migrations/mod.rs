use fuzion_commons::config::DatabaseConfigError;
use fuzion_commons::db::DeadpoolPoolError;
use fuzion_commons::migration::{MigrationError, Migrator};
use thiserror::Error;

use crate::server::config::FuzionVeritaConfig;

pub mod v0_1_0;

pub use v0_1_0::*;

pub async fn init(config: &FuzionVeritaConfig) -> Result<(), MigrationInitError> {
  if config.migrate {
    let db_pool = config.database.get_db_pool().await?;

    let db_client = db_pool.get().await?;

    let mut migrator = Migrator::new("verita", db_client, vec![Box::new(V0_1_0 {})]);

    migrator.migrate().await?
  }

  Ok(())
}

#[derive(Debug, Error)]
pub enum MigrationInitError {
  #[error(transparent)]
  DatabaseConfigError(#[from] DatabaseConfigError),
  #[error(transparent)]
  DeadpoolPoolError(#[from] DeadpoolPoolError),
  #[error("migration failed")]
  MigrationFailure,
  #[error(transparent)]
  MigrationError(#[from] MigrationError),
}
