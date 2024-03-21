use actix_web::*;

pub async fn login() -> Result<HttpResponse, Error> {
  Ok(HttpResponse::Ok().finish())
}
