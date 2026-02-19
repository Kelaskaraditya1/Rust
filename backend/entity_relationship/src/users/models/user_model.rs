use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug,Serialize,Deserialize)]
pub struct Users{
    
    user_id:Uuid,
    name:String,
    contact:String,
    email:String,
    username:String,
    password:String,
    created_at:chrono::NaiveDateTime

}