use axum::http::StatusCode;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use chrono::{DateTime, NaiveDateTime};

use crate::{dto::request_dto::UserRequestDto, exception::global_exception_handler::ApiError};


fn parse_created_at(raw: &str) -> Result<NaiveDateTime, String> {
    if let Ok(epoch_ms) = raw.parse::<f64>() {
        let epoch_secs = (epoch_ms / 1000.0) as i64;
        if let Some(dt) = DateTime::from_timestamp(epoch_secs, 0) {
            return Ok(dt.naive_utc());
        }
        return Err(format!("Epoch value out of range: {}", raw));
    }

    NaiveDateTime::parse_from_str(raw, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| format!("Cannot parse '{}' as datetime: {}", raw, e))
}

pub async fn insert_user(
    db: &DatabaseConnection,
    user_request: UserRequestDto,
) -> Result<(), ApiError> {

    let created_at = parse_created_at(&user_request.created_at)
        .map_err(|e| ApiError {
            status_code: StatusCode::BAD_REQUEST,
            message: format!("Invalid datetime for user '{}': {}", user_request.user_id, e),
        })?;

    let user_model = entity::users::ActiveModel {
        user_id: Set(user_request.user_id),
        name: Set(user_request.name),
        email: Set(user_request.email),
        contact: Set(user_request.contact),
        username: Set(user_request.username),
        password: Set(user_request.password),
        created_at: Set(created_at),
        ..Default::default()
    };

    user_model.insert(db).await.map_err(|e| ApiError {
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        message: format!("DB insert failed: {}", e),
    })?;

    Ok(())
}