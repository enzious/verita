use actix_web::*;

mod login;
mod logout;

pub fn build() -> Scope {
  Scope::new("/session")
    .service(login::submit)
    .service(logout::submit)
}
