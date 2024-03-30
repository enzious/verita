use actix_web::*;
use fuzion_commons::db::PgClient;

use crate::dto::login::LoginInitRequest;
use crate::services::realm::RealmService;

#[post("/login")]
pub async fn init(
  db_client: PgClient<'_>,
  web::Json(body): web::Json<LoginInitRequest>,
) -> Result<HttpResponse, Error> {
  let LoginInitRequest { realm } = body;

  let realm = RealmService::get_realm_by_name_required(&db_client, &realm).await?;

  Ok(HttpResponse::Ok().json(realm))
}
