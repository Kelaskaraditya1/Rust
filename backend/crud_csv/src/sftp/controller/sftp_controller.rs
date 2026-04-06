use axum::{Router, routing::post};

use crate::sftp::service::sftp_service::{read_remote_file, write_to_file};

pub fn sftp_controller()->Router{

    Router::new()
        .route("/api/v1/sftp/read/{file_path}",post(read_remote_file))
        .route("/api/v1/sftp/write/{file_path}",post(write_to_file))

}