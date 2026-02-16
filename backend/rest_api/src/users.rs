use std::env;

use axum::{Json, http::StatusCode, response::IntoResponse};
use dotenvy::dotenv;
use entity::users::{self, Model};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, Database, DatabaseConnection};
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Users{
    pub user_id:Option<String>,
    pub name:String,
    pub email:String,
    pub contact:String,
    pub username:String,
    pub password:String,
    pub created_at:chrono::NaiveDateTime
}

#[derive(Debug,Serialize,Deserialize)]
pub struct UserCreateRequest{

    pub name:String,
    pub email:String,
    pub contact:String,
    pub username:String,
    pub password:String,
}

pub async fn create_user(Json(user_request):Json<UserCreateRequest>)->impl IntoResponse{

      dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database url not found");
    let db = Database::connect(database_url).await.unwrap();

    let user_model = users::ActiveModel{
        user_id:Set(uuid::Uuid::new_v4()),
        name:Set(user_request.name),
        email:Set(user_request.email),
        contact:Set(user_request.contact),
        username:Set(user_request.username),
        password:Set(user_request.password),
        created_at:Set(chrono::Utc::now().naive_utc())
    };

    let user_response = user_model.insert(&db).await.unwrap();
    println!("User created successfully: {:?}",user_response);

    (StatusCode::CREATED,Json(user_response))


}