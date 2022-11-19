use crate::use_case::update_value_by_key;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostKeyValuePathParams {
    key: String,
    value: String,
}

#[post("/key/{key}/value/{value}")]
pub async fn post_key_value(path_params: web::Path<PostKeyValuePathParams>) -> impl Responder {
    match update_value_by_key(path_params.key.clone(), path_params.value.clone()).await {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}
