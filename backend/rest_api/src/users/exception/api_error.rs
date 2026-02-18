use axum::{ Json, http::{StatusCode, header}, response::IntoResponse };
use serde_json::json;

#[derive(Debug)]
pub struct ApiError{

    pub message:String,
    pub status_code:StatusCode

}

impl IntoResponse for ApiError{
    fn into_response(self) -> axum::response::Response {
        let status_code = self.status_code;
        (
            status_code,  // Status code
            [
                (header::CONTENT_TYPE,"application/json")
            ],  // List of Headers
            Json(
                json!({
                    "message":self.message,
                    "status_code":self.status_code.as_u16()
                })
            )  // Response 
        ).into_response()
    }
}