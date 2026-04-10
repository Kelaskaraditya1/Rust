use std::{env};

use axum::{Json, Router, routing::{get, post}};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

use crate::{models::models::AppState, service::service::{get_consumer_data, get_users}};

pub mod exception;
pub mod keys;  
pub mod models;
pub mod service;

#[derive(Debug,Serialize,Deserialize)]
pub struct Health{

    pub status:String,
    pub health:String,
    pub message:String

}

async fn health()->Json<Health>{

    Json(
        Health { status: "Ok".to_string(),
         health: "Healthy".to_string(),
          message: "Rust Service available on port: ".to_string() 
        }
    )
    
}

#[tokio::main]
async fn main() {

    let app_state = AppState::new();

    let cors_layer = CorsLayer::new()
        .allow_origin([])
        .allow_headers(Any)
        .allow_methods([Method::GET,Method::POST,Method::PUT,Method::PATCH,Method::DELETE,Method::OPTIONS]);

    dotenvy::dotenv().ok();
    let port = env::var("PORT").expect("invalid key Port");
    println!("Consumer running on port:{}",port);

    let app = Router::new()
        .route("/api/v1/health", get(health))
        .route("/api/v1/users",get(get_users))
        .route("/api/v1/welcome/email", post(get_consumer_data))
        .layer(cors_layer)
        .with_state(app_state);

    let ip_address = keys::keys::IP_ADDRESS.to_string();

    let listener = tokio::net::TcpListener::bind(ip_address).await.unwrap();

    axum::serve(listener, app).await.unwrap();

}
