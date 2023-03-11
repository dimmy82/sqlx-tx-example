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

pub async fn update_by_key(
    param: Param, mut tx: Transaction<'_, Postgres>,
) -> Result<(u64, Transaction<'_, Postgres>)> {
    let result = if param.value == "NULL" {
        _update_null_by_key(param.key, &mut tx).await?
    } else {
        _update_value_by_key(param.key, param.value.clone(), &mut tx).await?
    };
    if param.value == "panic" {
        panic!("holy sh*t")
    }
    Ok((result, tx))
}

async fn _update_value_by_key(
    key: String, value: String, tx: &mut Transaction<'_, Postgres>,
) -> Result<u64> {
    let result =
        sqlx::query("UPDATE paper.paper_formula SET formula = $2 WHERE main_group_id = $1")
            .bind(key)
            .bind(value.clone())
            .execute(tx)
            .await?;
    Ok(result.rows_affected())
}

async fn _update_null_by_key(key: String, tx: &mut Transaction<'_, Postgres>) -> Result<u64> {
    let result =
        sqlx::query("UPDATE paper.paper_formula SET formula = NULL WHERE main_group_id = $1")
            .bind(key)
            .execute(tx)
            .await?;
    Ok(result.rows_affected())
}
