use axum::{Router, routing::post};

use crate::service::file_service::upload_file;

pub fn file_controller()->Router{

     Router::new()
        .route("/api/v1/file/upload",post(upload_file))
    
    
}