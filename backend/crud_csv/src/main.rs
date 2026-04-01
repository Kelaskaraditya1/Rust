use std::{env};

use axum::{Extension, Json, Router, routing::get};
use sea_orm::Database;
use serde::{Deserialize, Serialize};

use crate::{local::controller::file_controller::{file_controller}, keys::keys::DATABASE_URL};

pub mod keys;
pub mod exception;
pub mod local;

#[derive(Debug,Serialize,Deserialize)]
pub struct Health{
    status:String,
    health:String,
    message:String  
}

async fn health()->Json<Health>{
    Json(
        Health {
             status: "Ok".to_string(),
             health: "Healthy".to_string(),
              message: format!("app running on port:{}",env::var("PORT").expect("Invalid key PORT")) 
            }
    )
}

#[tokio::main]
async fn main(){

    let database_url = DATABASE_URL.to_string();
    let database_connection= Database::connect(database_url).await;

    if database_connection.is_ok(){
        println!("Connected to Database sucessfully!");
    }else{
        println!("Failed to connect to Database");
    }

    let app = Router::new()
        .route("/api/v1/health",get(health))
        .merge(file_controller())
        .layer(Extension(database_connection.unwrap()));

    let address = keys::keys::ADDRESS.to_string();
    let listner = tokio::net::TcpListener::bind(address).await.unwrap();

    println!("App started on port:{}",env::var("PORT").expect("Invalid Key POST"));

    axum::serve(listner,app).await.unwrap();
}

