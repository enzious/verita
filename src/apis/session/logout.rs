use actix_web::*;
use fuzion_commons::db::PgClient;

use crate::dao::realm::RealmId;
use crate::domain::cookies::SessionCookies;
use crate::services::session::SessionService;

#[post("/logout")]
pub async fn submit(
  db_client: PgClient<'_>,
  web::Query(Query { realm_id }): web::Query<Query>,
  mut session: SessionCookies,
) -> Result<HttpResponse, Error> {
  if let Some(identity) = session.remove_identity(realm_id) {
    SessionService::logout(&db_client, &identity).await?;

    return Ok(HttpResponse::Ok().cookie(session.to_cookie()?).finish());
  }

  Ok(HttpResponse::Ok().finish())
}

#[derive(Deserialize)]
pub struct Query {
  realm_id: RealmId,
}
