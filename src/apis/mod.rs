use actix_web::*;

mod hello;

pub fn build(config: &mut web::ServiceConfig) {
  config.service(web::resource("/").route(web::get().to(hello::hello)));
}
