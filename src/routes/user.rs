use actix_web::web;

use crate::handlers::user::{get_user, create_user};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("", web::get().to(get_user))
            .route("", web::post().to(create_user)),
    );
}
