use actix_web::*;

mod login;

pub fn build() -> Scope {
  Scope::new("/session")
    .service(login::login_init)
    .service(login::login)
}
