use axum::{Router, routing::{get, post}};

use crate::users::service;

pub fn user_router()->Router{

    let router = Router::new()
        .route("/api/v1/users/signup",post(service::user_service::signup))
        .route("/api/v1/users/login", post(service::user_service::login))
        .route("/api/v1/users/{user_id}", get(service::user_service::get_user_by_id))
        .route("/api/v1/users",get(service::user_service::get_all_users));

    router


}