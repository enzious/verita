use fuzion_commons::db::PgClient;

use crate::dao::credential::{CredentialConfig, CredentialConfigId};

use super::RepoError;

pub struct CredentialRepo;

impl CredentialRepo {
  pub async fn get_credential_config_by_id(
    db_client: &PgClient<'_>,
    id: CredentialConfigId,
  ) -> Result<Option<CredentialConfig>, RepoError> {
    let stmt = db_client
      .prepare_cached(STMT_GET_CREDENTIAL_CONFIG_BY_ID)
      .await?;
    let rows = db_client.query(&stmt, &[&id]).await?;
    Ok(rows.get(0).map(|row| row.into()))
  }
}

static STMT_GET_CREDENTIAL_CONFIG_BY_ID: &'static str = r#"

SELECT *
FROM verita.credential_config
WHERE id = $1

"#;
