use actix_web::*;
use fuzion_commons::db::PgClient;

use crate::domain::cookies::SessionCookies;
use crate::dto::login::LoginRequest;
use crate::services::session::SessionService;

#[post("/login")]
pub async fn submit(
  db_client: PgClient<'_>,
  web::Json(body): web::Json<LoginRequest>,
  _: SessionCookies,
) -> Result<HttpResponse, Error> {
  let LoginRequest {
    realm_id,
    user,
    password,
  } = body;

  SessionService::login(&db_client, realm_id, &user, &password).await?;

  Ok(HttpResponse::Ok().finish())
}
