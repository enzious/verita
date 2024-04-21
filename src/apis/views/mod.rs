use actix_web::*;

mod login;
mod shell;

pub fn build() -> Scope {
  Scope::new("/views")
    .service(login::init)
    .service(shell::init)
}
