use axum::{Router, routing::post};

use crate::{local::service::file_service::{upload_file, write_file}};

pub fn file_controller()->Router{

     Router::new()
        .route("/api/v1/local/upload",post(upload_file))
        .route("/api/v1/local/download", post(write_file))
    
    
}