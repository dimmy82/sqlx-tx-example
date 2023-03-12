use crate::use_case::{execute_use_case_in_tx, update_main_and_create_log, Param};
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostKeyValuePathParams {
    key: String,
    value: String,
}

#[get("/key/{key}/value/{value}")]
pub async fn post_key_value(path_params: web::Path<PostKeyValuePathParams>) -> impl Responder {
    match execute_use_case_in_tx(
        update_main_and_create_log,
        Param {
            key: path_params.key.clone(),
            value: path_params.value.clone(),
        },
    )
    .await
    {
        Ok(_) => HttpResponse::Created().body("Succeed"),
        Err(_) => HttpResponse::InternalServerError().body("Failed"),
    }
}
