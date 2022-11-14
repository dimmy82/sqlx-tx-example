use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostKeyValuePathParams {
    key: String,
    value: String,
}

#[post("/key/{key}/value/{value}")]
pub async fn post_key_value(path_params: web::Path<PostKeyValuePathParams>) -> impl Responder {
    HttpResponse::Created()
}
