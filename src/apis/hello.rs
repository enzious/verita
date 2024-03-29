use actix_web::*;

#[get("/")]
pub async fn hello() -> Result<HttpResponse, Error> {
  Ok(HttpResponse::Ok().body("hello world"))
}
