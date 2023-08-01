use std::sync::Arc;

use crate::data::database::Database;

pub async fn delete_expired_sessions(database: Arc<Database>) -> Result<(), anyhow::Error> {
    let conn = database.connect().await?;
    conn.frontend_accounts().delete_expired_sessions().await?;

    Ok(())
}
