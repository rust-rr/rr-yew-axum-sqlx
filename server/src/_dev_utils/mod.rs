use tokio::sync::OnceCell;
use tracing::info;

mod dev_db;

/// Initialize environment for local development.
/// (for early development, will be called from main()).
/// docker exec -it -u postgres pgsql-dev psql
pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        info!("{:<12} - init_dev_all()", "FOR-DEV-ONLY");

        dev_db::init_dev_db().await.unwrap();
    })
    .await;
}
