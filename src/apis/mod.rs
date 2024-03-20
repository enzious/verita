use actix_files::Files;
use actix_web::*;

mod hello;

pub fn build(config: &mut web::ServiceConfig) {
  config
    .service(Scope::new("/api").service(web::resource("/").route(web::get().to(hello::hello))))
    .service(Files::new("/", "./static/").prefer_utf8(true));
}
