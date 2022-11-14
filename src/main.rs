mod rest;

use crate::rest::post_key_value;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use std::env;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "info");
    json_env_logger::init();
    json_env_logger::panic_hook();

    // let DB_POOL = create_pool().await;
    // RND_CQRS_DB_POOL.set(rnd_cqrs_db_pool).unwrap();

    HttpServer::new(|| App::new().wrap(Logger::default()).service(post_key_value))
        .bind("0.0.0.0:12345")?
        .run()
        .await
}
