use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct UserRegisterRequest{
    pub name:String,
    pub contact:String,
    pub email:String,
    pub username:String,
    pub password:String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct LoginRequest{
    pub username:String,
    pub password:String
}