use actix_web::*;

mod login;

pub fn build() -> Scope {
  Scope::new("/views").service(login::init)
}
