use axum::{Extension, Json, debug_handler, extract::Path, http::StatusCode};
use entity::post::{self, Column, Model};
use sea_orm::{ColumnTrait, QueryFilter};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};
use uuid::Uuid;
use entity::prelude::Post;
use entity::prelude::Users;

use crate::{exception::global_expection::ApiError, posts::dto::{request::CreatePostRequest, response::PostResponse}};

#[debug_handler]
pub async fn create_post(
    Extension(db):Extension<DatabaseConnection>,
    Json(post):Json<CreatePostRequest>
)->Result<Json<Model>,ApiError>{


let post_model = post::ActiveModel{

    post_id:Set(Uuid::new_v4()),
    title:Set(post.title.to_owned()),
    content:Set(post.content.to_string()),
    created_at:Set(chrono::Utc::now().naive_utc().to_owned()),
    media_url:Set(post.media_url.to_owned()),
    user_id:Set(post.user_id.to_owned())

};

let response = post_model.insert(&db)
    .await
    .map_err(
        |error| ApiError{
            status_code:StatusCode::INTERNAL_SERVER_ERROR,
            message:error.to_string()
        }
    )?;

    return Ok(Json(response));

}

#[debug_handler]
pub async fn get_posts(
    Path(user_id):Path<Uuid>,
    Extension(db):Extension<DatabaseConnection>
)->Result<Json<PostResponse>,ApiError>{

    let user_with_id = Users::find_by_id(user_id)
        .one(&db)
        .await
        .map_err(
            |error| ApiError{
                status_code:StatusCode::INTERNAL_SERVER_ERROR,
                message:error.to_string()
            }
        )?
        .ok_or(
           ApiError{
            status_code:StatusCode::INTERNAL_SERVER_ERROR,
            message:"enter proper userId".to_string()
           } 
        )?;

        // let posts = user_with_id.find_related(Post)
        //     .all(&db)
        //     .await
        //     .map_err(
        //      |error| ApiError{
        //         status_code:StatusCode::INTERNAL_SERVER_ERROR,
        //         message:error.to_string()
        //      }   
        //     )?;

        let posts = Post::find()
            .filter(Column::UserId.eq(user_id))
            .all(&db)
            .await
            .map_err(
                |error| ApiError{
                    status_code:StatusCode::BAD_REQUEST,
                    message:error.to_string()
                }
            )?;

        let response = PostResponse{
            user:user_with_id,
            posts_list:posts
        };

        Ok(Json(response))

}