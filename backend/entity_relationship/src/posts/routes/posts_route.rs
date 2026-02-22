use axum::{Router,  routing::{get, post}};
use crate::posts::service::posts_service::{create_post, get_posts};


pub fn post_router()->Router{

    let router = Router::new()
        .route("/api/v1/post/create", post(create_post))
        .route("/api/v1/post/{user_id}",get(get_posts));

    router

}