use actix_web::*;

mod login;

pub fn build() -> Scope {
  Scope::new("/session").service(web::resource("/login").route(web::post().to(login::login)))
}
