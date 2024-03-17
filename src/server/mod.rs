use actix_web::dev::Server;
use actix_web::{App, HttpServer};

use crate::apis;

pub mod config;

use self::config::FuzionVeritasConfig;

pub async fn build(server_config: &FuzionVeritasConfig) -> Result<Server, ()> {
  let srv = {
    let _server_config = server_config.to_owned();

    HttpServer::new(move || {
      let app = App::new();

      app.configure(|config| apis::build(config))
    })
  }
  .shutdown_timeout(5u64)
  .bind(format!(
    "{}:{}",
    &server_config.http.host, &server_config.http.port
  ))
  .expect("Cannot bind.")
  .run();

  Ok(srv)
}
