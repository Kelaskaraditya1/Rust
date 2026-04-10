use axum::{Router, routing::post};

use crate::{ service::service::signup};

pub fn user_router()->Router{

    Router::new()
        .route("/api/v1/auth/signup",post(signup))


}