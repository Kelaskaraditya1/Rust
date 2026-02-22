use axum::{Extension, Json, Router, debug_handler, http::Method, routing::get};
use sea_orm::Database;
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

use crate::users::routes;

pub mod keys;
pub mod users;
pub mod exception;
pub mod posts;
pub mod comments;

#[tokio::main]
async fn main(){

    // Database connection

    let database_url = (*keys::keys::DATABASE_URL).clone();
    let database_connection = Database::connect(&database_url).await.unwrap();

    // Cors Policy

    let cors_policy = CorsLayer::new()
        .allow_methods([Method::GET,Method::POST,Method::PUT,Method::PATCH,Method::DELETE,Method::OPTIONS])
        .allow_headers(Any)
        .allow_origin(Any);

    // Base Router

    let router = Router::new()
        .route(
            "/api/v1/health",
            get(health)
        )
        .merge(routes::user_routes::user_router())
        .merge(posts::routes::posts_route::post_router())
        .layer(Extension(database_connection))
        .layer(cors_policy);

        let ip_address = (*keys::keys::IP_ADDRESS).clone();

        let listner = tokio::net::TcpListener
            ::bind(&ip_address)
            .await
            .unwrap();

        axum::serve(listner,router).await.unwrap();

}

#[derive(Debug,Serialize,Deserialize)]
pub struct Health{
    status:String,
    health:String
}

#[debug_handler]
pub async fn health()->Json<Health>{

    println!("accessed health route");

    Json(
        Health {
            status:"ok".to_owned(),
            health:"healthy".to_owned()
        }
    )

}