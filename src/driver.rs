use color_eyre::Result;
use sqlx::postgres::PgRow;
use sqlx::{Postgres, Row, Transaction};

pub async fn select_main(id: String, tx: &mut Transaction<'_, Postgres>) -> Result<Option<String>> {
    let result: Option<String> =
        sqlx::query("SELECT value FROM paper.main WHERE id = $1 FOR UPDATE")
            .bind(id)
            .map(|row: PgRow| row.get("value"))
            .fetch_optional(tx)
            .await?;
    Ok(result)
}

pub async fn update_main(
    id: String, value: String, tx: &mut Transaction<'_, Postgres>,
) -> Result<u64> {
    let result = sqlx::query("UPDATE paper.main SET value = $2 WHERE id = $1")
        .bind(id)
        .bind(value)
        .execute(tx)
        .await?;
    Ok(result.rows_affected())
}

pub async fn update_main_null(id: String, tx: &mut Transaction<'_, Postgres>) -> Result<u64> {
    let result = sqlx::query("UPDATE paper.main SET value = NULL WHERE id = $1")
        .bind(id)
        .execute(tx)
        .await?;
    Ok(result.rows_affected())
}

pub async fn insert_main(
    id: String, value: String, tx: &mut Transaction<'_, Postgres>,
) -> Result<u64> {
    let result = sqlx::query("INSERT INTO paper.main (id, value) VALUES ($1, $2)")
        .bind(id)
        .bind(value)
        .execute(tx)
        .await?;
    Ok(result.rows_affected())
}

pub async fn insert_log(value: String, tx: &mut Transaction<'_, Postgres>) -> Result<u64> {
    let result = sqlx::query("INSERT INTO paper.log (value) VALUES ($1)")
        .bind(value)
        .execute(tx)
        .await?;
    Ok(result.rows_affected())
}
