use actix_web::*;
use fuzion_commons::db::PgClient;

use crate::dto::login::LoginRequest;
use crate::services::session::SessionService;

#[post("/login")]
pub async fn submit(
  db_client: PgClient<'_>,
  web::Json(body): web::Json<LoginRequest>,
) -> Result<HttpResponse, Error> {
  let LoginRequest {
    realm_id,
    user,
    password,
  } = body;

  SessionService::login(&db_client, realm_id, &user, &password).await?;

  Ok(HttpResponse::Ok().finish())
}
