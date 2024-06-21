use actix_web::{web, HttpResponse, Responder};
use crate::models::user::User;
use log::info;
use tracing::instrument;

#[instrument]
pub async fn get_user() -> impl Responder {
    info!("Handling get_user request");
    let user = User {
        id: 1,
        name: String::from("John Doe"),
    };
    HttpResponse::Ok().json(user)
}

#[instrument]
pub async fn create_user(user: web::Json<User>) -> impl Responder {
    info!("Handling create_user request");
    HttpResponse::Ok().json(user.into_inner())
}
