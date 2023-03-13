use crate::driver::{insert_log, insert_main, select_main, update_main, update_main_null};
use crate::DB_POOL;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use kv_log_macro as log_kv;
use sqlx::{Postgres, Transaction};
use std::fmt::Debug;
use std::future::Future;

pub async fn execute_use_case_in_tx<'a, T, R, F, FR>(use_case: F, use_case_param: T) -> Result<R>
where
    R: Debug,
    FR: Future<Output = Result<(R, Transaction<'a, Postgres>)>>,
    F: FnOnce(T, Transaction<'a, Postgres>) -> FR,
{
    match DB_POOL.get() {
        Some(db_pool) => {
            let tx = db_pool.begin().await?;
            let result = use_case(use_case_param, tx).await;
            match result {
                Ok((r, tx)) => {
                    tx.commit().await?;
                    log_kv::info!("UseCase OK, db ({:?})rows affected", r);
                    Ok(r)
                }
                Err(error) => {
                    /*
                    A transaction should end with a call to commit or rollback.
                    If neither are called before the transaction goes out-of-scope, rollback is called.
                    In other words, rollback is called on drop if the transaction is still in-progress.
                    ということで、rollbackを明確に呼び出さなくても、スコープから抜けるときまだ生きていればrollbackされる。
                    use_caseの途中でpanicされてもrollbackされることも検証済み
                    */
                    // tx.rollback().await?;
                    log_kv::error!("UseCase NG ({:?})", error);
                    Err(error)
                }
            }
        }
        None => Err(eyre!("DB Pool is not set !")),
    }
}

pub struct Param {
    pub key: String,
    pub value: String,
}

pub async fn update_main_and_create_log(
    param: Param, mut tx: Transaction<'_, Postgres>,
) -> Result<(u64, Transaction<'_, Postgres>)> {
    let value = select_main(param.key.as_str(), &mut tx).await?;
    let result = match value {
        Some(v) => {
            let result = if param.value.as_str() == "NULL" {
                update_main_null(param.key.as_str(), &mut tx).await?
            } else {
                update_main(param.key.as_str(), param.value.as_str(), &mut tx).await?
            };
            insert_log(
                format!(
                    "update id:[{:?}] value:[{:?} -> {:?}]",
                    param.key, v, param.value
                )
                .as_str(),
                &mut tx,
            )
            .await?;
            result
        }
        None => {
            let result = insert_main(param.key.as_str(), param.value.as_str(), &mut tx).await?;
            insert_log(
                format!("insert id:[{:?}] value:[{:?}]", param.key, param.value).as_str(),
                &mut tx,
            )
            .await?;
            result
        }
    };

    if param.value.as_str() == "panic" {
        panic!("holy sh*t !!!")
    }

    Ok((result, tx))
}
