use async_trait::async_trait;
use deadpool_postgres::tokio_postgres;
use fuzion_commons::migration::Migration;
use fuzion_commons::version::Version;

const UP_SQL: &'static str = include_str!("up.sql");

pub struct V0_1_0 {}

#[async_trait]
impl Migration for V0_1_0 {
  fn version(&self) -> Version {
    Version(0, 1, 0)
  }

  async fn do_migration(&self, conn: &mut tokio_postgres::Transaction<'_>) -> Result<(), ()> {
    conn.batch_execute(UP_SQL).await.map_err(|e| {
      error!("Migration failed: {:?}", e);
      ()
    })?;

    Ok(())
  }
}
