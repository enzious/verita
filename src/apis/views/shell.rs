use actix_web::*;
use fuzion_commons::db::PgClient;

use crate::dto::shell::ShellInit;
use crate::repos::realm::RealmRepo;

#[get("/shell")]
pub async fn init(db_client: PgClient<'_>) -> Result<HttpResponse, Error> {
  let realms = RealmRepo::get_realms(&db_client).await?;

  Ok(HttpResponse::Ok().json(ShellInit { realms }))
}
