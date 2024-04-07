use actix_web::*;
use fuzion_commons::db::PgClient;

use crate::dao::realm::RealmId;
use crate::domain::cookies::SessionCookies;
use crate::domain::jwt::VeritaJwtKey;

#[get("/logout")]
pub async fn post(
  _db_client: PgClient<'_>,
  key: web::Data<VeritaJwtKey>,
  web::Query(Query { realm }): web::Query<Query>,
  mut session: SessionCookies,
) -> Result<HttpResponse, Error> {
  if let Some(_identity) = session.remove_identity(realm) {
    // SessionService::logout(&db_client, &identity).await?;

    return Ok(HttpResponse::Ok().cookie(session.to_cookie(&key)?).finish());
  }

  Ok(HttpResponse::Ok().finish())
}

#[derive(Deserialize)]
pub struct Query {
  realm: RealmId,
}
