#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use actix_rt::signal;
use fuzion_commons::logging;

pub mod apis;
pub mod config;
pub mod migrations;
pub mod server;

use server::config::FuzionVeritaConfig;

#[actix_web::main]
async fn main() -> Result<(), ()> {
  let server_config = FuzionVeritaConfig::load();
  logging::init(&server_config.logging);

  {
    let db_pool = server_config
      .database
      .get_db_pool()
      .await
      .expect("Failed to initialize DB pool.");

    let db_conn = db_pool.get().await.expect("");
    migrations::init(&server_config, db_conn).await;
  }

  let srv = server::build(&server_config).await.map_err(|_| ())?;

  let srv_handle = srv.handle();

  actix_rt::spawn(srv);

  signal::ctrl_c().await.map_err(|_| ())?;

  srv_handle.stop(true).await;

  Ok(())
}
