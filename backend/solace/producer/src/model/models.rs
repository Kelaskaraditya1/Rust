use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct UsersRequest{

    pub name:String,
    pub email:String,
    pub contact:String,
    pub username:String,
    pub password:String

}

#[derive(Debug,Serialize,Clone)]
pub struct Users{

    pub user_id:String,
    pub name:String,
    pub email:String,
    pub contac:String,
    pub username:String,
    pub password:String,
    pub created_at:chrono::NaiveDateTime

}


#[derive(Debug,Serialize,Deserialize)]
pub struct UserRegisteredEvent {
    pub event_id: String,
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub registered_at: String,
}