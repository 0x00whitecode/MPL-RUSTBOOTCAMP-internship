use actix_web::web;
use crate::handlers::*;


pub fn user_routes(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/users")
        .route("", web::post().to(create_user))
        .route("", web::get().to(get_users))
        .route("/{id}", web::get().to(get_user))
        .route("/{id}", web::put().to(update_user))
        .route("/{id}", web::delete().to(delete_user)),
    );
}