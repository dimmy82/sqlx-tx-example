mod rest;
mod usecase;

use crate::rest::post_key_value;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use color_eyre::eyre::Result;
use once_cell::sync::OnceCell;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

pub static DB_POOL: OnceCell<PgPool> = OnceCell::new();

#[actix_web::main]
pub async fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "info");
    color_eyre::install()?;
    json_env_logger::init();
    json_env_logger::panic_hook();

    DB_POOL.set(create_pool().await?).unwrap();

    Ok(
        HttpServer::new(|| App::new().wrap(Logger::default()).service(post_key_value))
            .bind("0.0.0.0:12345")?
            .run()
            .await?,
    )
}

async fn create_pool() -> Result<PgPool> {
    Ok(PgPoolOptions::new()
        .max_connections(10)
        .connect("postgres://ibex:ibex@localhost:30395/ip")
        .await?)
}
