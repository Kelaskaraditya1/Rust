/*
Route Level: for creating routes we have to add using:
    cargo add tower-http --features cors

    step 1) create a function whose return type is Router, now add the Cors layer so for doing that

        let cors_policy = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE, Method::OPTIONS])
            .allow_origin(Any);

    step 2) add the routes:
            Router::new()
                .route("/api/v1/users/signup",post(signup))
                .route("/api/v1/users/login",post(login))
                .layer(cors_policy) // add the cors policy

    and now add/register this router() method in main.rs using merge(router()) and call usign() and not by only name.


*/

use axum::{Router, http::Method, middleware, routing::{delete, get, post, put}};
use tower_http::cors::{Any, CorsLayer};

use crate::users::service::{guards::guard, service::{delete_user, get_all_users, login, signup, update_user}};

pub fn router()->Router{

    let cors_policy = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE, Method::OPTIONS])
        .allow_origin(Any)
        .allow_headers(Any);

    // Public routes (no authentication required)
    let public_routes = Router::new()
        .route("/api/v1/users/signup", post(signup))
        .route("/api/v1/users/login", post(login));

    // Protected routes (JWT authentication required)
    let protected_routes = Router::new()
        .route("/api/v1/users/update/{user_id}", put(update_user))
        .route("/api/v1/users/delete/{user_id}", delete(delete_user))
        .route("/api/v1/users", get(get_all_users))
        .layer(middleware::from_fn(guard));

    public_routes
        .merge(protected_routes)
        .layer(cors_policy)
}