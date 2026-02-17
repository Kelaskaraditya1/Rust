use uuid::Uuid;
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Users{
    pub user_id:Uuid,
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

#[derive(Debug,Serialize,Deserialize)]
pub struct LoginRequest{
    pub username:String,
    pub password:String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct UpdateUserRequest{

    pub name:String,
    pub email:String,
    pub contact:String,
    pub username:String

} 