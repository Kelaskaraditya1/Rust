use axum::{Extension, Json, debug_handler, http::StatusCode};
use chrono::{DateTime, NaiveDateTime};
use entity::users::{Column, Model};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{exception::global_exception_handler::ApiError, sftp::service::sftp_service::User};

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

#[debug_handler]
pub async fn insert_user(
    Extension(db):Extension<DatabaseConnection>,
    Json(user_request):Json<User>
)->Result<(),ApiError>{

    let user_with_contact = entity::prelude::Users::find()
        .filter(Column::Contact.eq(&user_request.contact))
        .one(&db)
        .await
        .map_err(
            |error| ApiError{
                status_code:StatusCode::BAD_REQUEST,
                message:format!("Contact already exist: {}",error.to_string())
            }
        )?;

        if user_with_contact.is_some(){
            return Err(
                ApiError{
                    status_code:StatusCode::BAD_REQUEST,
                    message:"contact already taken".to_string()
                }
            );
        }

        let user_with_email = entity::prelude::Users::find()
            .filter(Column::Email.eq(&user_request.email))
            .one(&db)
            .await
            .map_err(
                |error| ApiError{
                status_code:StatusCode::BAD_REQUEST,
                message:format!("Email already exist: {}",error.to_string())
            }
        )?;

        if user_with_email.is_some(){
            return Err(
                ApiError{
                    status_code:StatusCode::BAD_REQUEST,
                    message:"email already taken".to_string()
                }
            );
        }

        let user_with_username = entity::prelude::Users::find()
            .filter(Column::Username.eq(&user_request.username))
            .one(&db)
            .await
            .map_err(
                |error| ApiError{
                status_code:StatusCode::BAD_REQUEST,
                message:format!("Username already exist: {}",error.to_string())
            }
        )?;

        if user_with_username.is_some(){
            return Err(
                ApiError{
                    status_code:StatusCode::BAD_REQUEST,
                    message:"username already taken".to_string()
                }
            );
        }

        let created_at = parse_created_at(&user_request.created_at)
            .map_err(
                |error| ApiError{
                    status_code:StatusCode::INTERNAL_SERVER_ERROR,
                    message:format!("unable to parse created_at:{}",error)
                }
            )?;

        let user = entity::users::ActiveModel{
            user_id:Set(user_request.user_id),
            name:Set(user_request.name),
            contact:Set(user_request.contact),
            email:Set(user_request.email),
            username:Set(user_request.username),
            password:Set(user_request.password),
            created_at:Set(created_at),

        };

        user.insert(&db)
            .await
            .map_err(
                |error| ApiError{
                    status_code:StatusCode::INTERNAL_SERVER_ERROR,
                    message:format!("Error while entering record: {}",error.to_string())
                } 
            )?;

        Ok(())

}

pub async fn  read_users(
    Extension(db):Extension<DatabaseConnection>
)->Result<Vec<Model>,ApiError>{

    let users = entity::prelude::Users::find()
        .all(&db)
        .await
        .map_err(
            |error| ApiError{
                status_code:StatusCode::INTERNAL_SERVER_ERROR,
                message:format!("failed to find users: {}",error)
            }
        )?;

        Ok(users)

        

}