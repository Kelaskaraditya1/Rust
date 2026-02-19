use axum::{Extension, Json, debug_handler, extract::Path, http::StatusCode};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;
use crate::{exception::global_expection::ApiError, users::dto::request::{LoginRequest, UserRegisterRequest}};
use entity::{prelude::Users, users::{self, Column, Model}};

#[debug_handler]
pub async fn signup(
    Extension(db):Extension<DatabaseConnection>,
    Json(user_request):Json<UserRegisterRequest>
)->Result<Json<Model>,ApiError>{ 

    let user_with_contact = Users::find()
        .filter(
            Column::Contact.eq(&user_request.contact)
        )
        .one(&db)
        .await
        .map_err(
            |error| ApiError{
                message:error.to_string(),
                status_code:StatusCode::BAD_REQUEST
            }
        )?;

        if user_with_contact.is_some(){

            return Err(
                ApiError{
                    message:"contact already taken".to_owned(),
                    status_code:StatusCode::BAD_REQUEST
                }
            );
        }

        let user_with_email = Users::find()
            .filter(
                Column::Email.eq(&user_request.email)
            )
            .one(&db)
            .await
            .map_err(
                |error| ApiError{
                    status_code:StatusCode::BAD_REQUEST,
                    message:error.to_string()
                }
            )?;

            if user_with_email.is_some(){
                return Err(
                    ApiError {
                        message:"email already taken".to_owned(),
                        status_code:StatusCode::BAD_REQUEST
                     }
                );
            }

        let user_with_username = Users::find()
            .filter(
                Column::Username.eq(&user_request.username)
            )
            .one(&db)
            .await
            .map_err(
                |error| ApiError{
                    status_code:StatusCode::BAD_REQUEST,
                    message:error.to_string()
                }
            )?;

            if user_with_username.is_some(){
                return Err(
                    ApiError {
                        status_code:StatusCode::BAD_REQUEST,
                        message:"username already taken".to_string()
                    }
                );
            }

            let user = users::ActiveModel{
                user_id:Set(Uuid::new_v4()),
                name:Set(user_request.name.to_owned()),
                contact:Set(user_request.contact.to_owned()),
                email:Set(user_request.email.to_owned()),
                username:Set(user_request.username.to_owned()),
                password:Set(user_request.password.to_owned()),
                created_at:Set(chrono::Utc::now().naive_local().to_owned())
            };

            let user_response = user.insert(&db)
                .await
                .unwrap();

            Ok(
                Json(
                    user_response
                )
            )

}

#[debug_handler]
pub async fn login(
    Extension(db):Extension<DatabaseConnection>,
    Json(login_request):Json<LoginRequest>
)->Result<Json<Model>,ApiError>{

    let user = Users::find()
        .filter(
            Condition::any()
                .add(Column::Email.eq(&login_request.username))   
                .add(Column::Username.eq(&login_request.username))
        )
        .filter(Column::Password.eq(login_request.password))
        .one(&db)
        .await
        .unwrap();

    if user.is_some(){
        return Ok(
            Json(user.unwrap())
        );
    }else{
        return Err(
            ApiError{
                status_code:StatusCode::UNAUTHORIZED,
                message:"Invalid username or password".to_string()
            }
        )
    }
}

#[debug_handler]
pub async fn get_user_by_id(
    Extension(db):Extension<DatabaseConnection>,
    Path(user_id):Path<Uuid>
)->Result<Json<Model>,ApiError>{

    let user_with_id = Users::find_by_id(user_id)
        .one(&db)
        .await
        .map_err(
           |error| ApiError{
                status_code:StatusCode::BAD_REQUEST,
                message:error.to_string()
            }
        )?;

        if user_with_id.is_some(){
            Ok(
                Json(user_with_id.unwrap())
            )
        }else{
            Err(
                ApiError {
                    status_code:StatusCode::BAD_REQUEST,
                    message:"Invalid userid".to_string()
                }
            )
        }

}

pub async fn get_all_users(
    Extension(db):Extension<DatabaseConnection>
)->Result<Json<Vec<Model>>,ApiError>{
    let user_list = Users::find()
        .all(&db)
        .await
        .unwrap();

    Ok(
        Json(user_list)
    )
}