// use crate::DB_POOL;
// use color_eyre::eyre::eyre;
// use color_eyre::Result;
// use kv_log_macro as log_kv;
// use sqlx::{Postgres, Transaction};
// use std::future::Future;
//
// async fn execute_in_tx<F, R, T>(use_case_in_tx: F) -> Result<T>
// where
//     F: for<'a> FnOnce(&'a Transaction<'a, Postgres>) -> R,
//     R: Future<Output = Result<T>>,
// {
//     match DB_POOL.get() {
//         Some(db_pool) => {
//             let tx = db_pool.begin().await?;
//             match use_case_in_tx(&tx).await {
//                 Ok(result) => {
//                     tx.commit().await?;
//                     log_kv::info!("UseCase OK");
//                     Ok(result)
//                 }
//                 Err(error) => {
//                     tx.rollback().await?;
//                     log_kv::error!("UseCase NG ({:?})", error);
//                     Err(error)
//                 }
//             }
//         }
//         None => Err(eyre!("DB Pool is not set !")),
//     }
// }
//
// pub async fn update_value_by_key(key: String, value: String) -> Result<String> {
//     Ok(execute_in_tx(move |tx| _update_value_by_key(key, value, tx)).await?)
// }
//
// async fn _update_value_by_key<'a>(
//     key: String, value: String, tx: &'a Transaction<'a, Postgres>,
// ) -> Result<String> {
//     unimplemented!()
// }
