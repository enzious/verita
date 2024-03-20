use actix_web::*;

pub async fn hello() -> Result<HttpResponse, Error> {
  Ok(HttpResponse::Ok().body("hello world"))
}
