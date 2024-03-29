use actix_web::*;
use fuzion_commons::db::PgClient;

use crate::dto::login::{LoginInitRequest, LoginRequest};
use crate::services::realm::RealmService;
use crate::services::session::SessionService;

#[post("/login/init")]
pub async fn login_init(
  db_client: PgClient<'_>,
  web::Json(body): web::Json<LoginInitRequest>,
) -> Result<HttpResponse, Error> {
  let LoginInitRequest { realm } = body;

  let realm = RealmService::get_realm_by_name_required(&db_client, &realm).await?;

  Ok(HttpResponse::Ok().json(realm))
}

#[post("/login")]
pub async fn login(
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
