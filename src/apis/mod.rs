use actix_files::Files;
use actix_web::*;

mod hello;
mod session;
mod views;

pub fn build(config: &mut web::ServiceConfig) {
  config
    .service(
      Scope::new("/api")
        .service(session::build())
        .service(views::build())
        .service(hello::hello),
    )
    .service(Files::new("/", "./static/").prefer_utf8(true));
}
