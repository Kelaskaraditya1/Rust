use axum::{Json, response::IntoResponse};
use reqwest::{StatusCode, header};
use serde_json::json;

#[derive(Debug)]
pub struct ApiError{
    pub status_code:StatusCode,
    pub message:String
}

impl IntoResponse for ApiError{
    fn into_response(self) -> axum::response::Response {
        (
            self.status_code,
            [
                (header::CONTENT_TYPE,"application/json")
            ],
            Json(
                json!({
                    "status_code":self.status_code.as_u16(),
                    "message":self.message
                })
            )
        ).into_response()
    }
}