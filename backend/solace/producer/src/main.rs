use std::{env};
use axum::{Json, Router, routing::get};
use serde::{Deserialize, Serialize};
use crate::{ controller::controller::user_router};

pub mod keys;
pub mod model;
pub mod service;
pub mod controller;
pub mod configuration;
pub mod exception;

#[derive(Debug,Serialize,Deserialize)]
pub struct Health{
    health:String,
    status:String,
    message:String
}

pub async fn health()->Json<Health>{
    let health =Health{
                health:"Healthy".to_string(),
                status:"Ok".to_string(),
                message:format!("App running on port: {}",env::var("PORT").expect("invalid key PORT"))
            };
    // println!("{:#?}",health);
    Json(health)
} 

#[tokio::main]
pub async fn main() {

    // let app_state = AppState::new();
    // dotenvy::dotenv().ok();
    // println!("{}", env::var("SOLACE_BASE_URL").expect("Invalid key from main"));

    dotenvy::dotenv().ok();
    let port = env::var("PORT").expect("invalid key Port");
    println!("Producer running on port:{}",port);

    let base_router = Router::new()
        .route("/api/v1/health",get(health))
        .merge(user_router());

    let ip_address = keys::keys::IP_ADDRESS.to_string();
    let listener = tokio::net::TcpListener::bind(ip_address).await.unwrap();

    axum::serve(listener,base_router).await.unwrap();

    

    
}
