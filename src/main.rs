#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use actix_rt::signal;
use fuzion_commons::logging;

pub mod apis;
pub mod migrations;
pub mod repos;
pub mod server;
pub mod services;

use server::config::FuzionVeritaConfig;

#[actix_web::main]
async fn main() -> Result<(), ()> {
  let config = FuzionVeritaConfig::load();

  logging::init(&config.logging);
  migrations::init(&config).await;

  let srv = server::build(&config).await?;

  let srv_handle = srv.handle();

  actix_rt::spawn(srv);

  signal::ctrl_c().await.map_err(|_| ())?;

  srv_handle.stop(true).await;

  Ok(())
}
