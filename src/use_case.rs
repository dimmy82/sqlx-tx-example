use crate::DB_POOL;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use kv_log_macro as log_kv;
use sqlx::{Postgres, Transaction};

pub async fn update_value_by_key(key: String, value: String) -> Result<String> {
    match DB_POOL.get() {
        Some(db_pool) => {
            let tx = db_pool.begin().await?;
            match _update_value_by_key(key, value, &tx).await {
                Ok(result) => {
                    tx.commit().await?;
                    log_kv::info!("UseCase OK");
                    Ok(result)
                }
                Err(error) => {
                    tx.rollback().await?;
                    log_kv::error!("UseCase NG ({:?})", error);
                    Err(error)
                }
            }
        }
        None => Err(eyre!("DB Pool is not set !")),
    }
}

async fn _update_value_by_key(
    key: String, value: String, tx: &Transaction<'_, Postgres>,
) -> Result<String> {
    unimplemented!()
}
