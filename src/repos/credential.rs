use fuzion_commons::db::PgClient;

use crate::dao::{
  credential::{CredentialConfig, CredentialConfigId},
  realm::RealmId,
};

use super::RepoError;

pub struct CredentialRepo;

impl CredentialRepo {
  pub async fn get_credential_config(
    db_client: &PgClient<'_>,
    id: CredentialConfigId,
  ) -> Result<Option<CredentialConfig>, RepoError> {
    let stmt = db_client.prepare_cached(STMT_GET_CREDENTIAL_CONFIG).await?;
    let rows = db_client.query(&stmt, &[&id]).await?;
    Ok(rows.get(0).map(|row| row.into()))
  }

  pub async fn get_credential_config_by_realm_id(
    db_client: &PgClient<'_>,
    realm_id: RealmId,
  ) -> Result<Option<CredentialConfig>, RepoError> {
    let stmt = db_client
      .prepare_cached(STMT_GET_CREDENTIAL_CONFIG_BY_REALM_ID)
      .await?;
    let rows = db_client.query(&stmt, &[&realm_id]).await?;
    Ok(rows.get(0).map(|row| row.into()))
  }

  pub async fn insert_credential_config(
    db_client: &PgClient<'_>,
    config: &CredentialConfig,
  ) -> Result<CredentialConfig, RepoError> {
    let stmt = db_client
      .prepare_cached(STMT_INSERT_CREDENTIAL_CONFIG)
      .await?;
    let rows = db_client
      .query(
        &stmt,
        &[
          &config.realm_id,
          &config.algorithm,
          &config.salt,
          &config.iterations,
          &config.created,
          &config.updated,
        ],
      )
      .await?;
    rows
      .get(0)
      .map(|row| row.into())
      .ok_or(RepoError::InternalError)
  }
}

static STMT_GET_CREDENTIAL_CONFIG: &'static str = r#"

SELECT *
FROM verita.credential_config
WHERE id = $1

"#;

static STMT_GET_CREDENTIAL_CONFIG_BY_REALM_ID: &'static str = r#"

SELECT *
FROM verita.credential_config
WHERE realm_id = $1
ORDER BY created DESC
LIMIT 1

"#;

static STMT_INSERT_CREDENTIAL_CONFIG: &'static str = r#"

INSERT INTO verita.credential_config
(realm_id, algorithm, salt, iterations, created, updated)
VALUES ($1, $2, $3, $4, $5, $6)
RETURNING *

"#;
