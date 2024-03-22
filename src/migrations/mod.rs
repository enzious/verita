use fuzion_commons::migration::Migrator;

use crate::server::config::FuzionVeritaConfig;

pub mod v0_1_0;

pub use v0_1_0::*;

pub async fn init(config: &FuzionVeritaConfig) {
  if config.migrate {
    let db_pool = config
      .database
      .get_db_pool()
      .await
      .expect("Failed to initialize database pool.");

    let db_client = db_pool
      .get()
      .await
      .expect("Failed to get database connection.");

    let mut migrator = Migrator::new("verita", db_client, vec![Box::new(V0_1_0 {})]);

    if let Err(err) = migrator.migrate().await {
      panic!("Failed to migrate database: {}", &err);
    }
  }
}
