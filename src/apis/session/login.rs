use actix_web::*;
use fuzion_commons::db::PgClient;

use crate::domain::cookies::SessionCookies;
use crate::dto::login::LoginRequest;
use crate::services::session::SessionService;

#[post("/login")]
pub async fn submit(
  db_client: PgClient<'_>,
  web::Json(body): web::Json<LoginRequest>,
  mut session: SessionCookies,
) -> Result<HttpResponse, Error> {
  let LoginRequest {
    realm_id,
    user,
    password,
  } = body;

  let identity = SessionService::login(&db_client, realm_id, &user, &password).await?;

  session.insert_identity(identity);

  Ok(HttpResponse::Ok().cookie(session.to_cookie()?).finish())
}
