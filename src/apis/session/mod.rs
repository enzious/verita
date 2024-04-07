use actix_web::*;

mod init;
mod login;
mod logout;

pub fn build() -> Scope {
  Scope::new("/session")
    .service(init::get)
    .service(login::post)
    .service(logout::post)
}
