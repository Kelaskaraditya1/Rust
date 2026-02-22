use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct CreatePostRequest{
    pub title:String,
    pub content:String,
    pub media_url:String,
    pub user_id:uuid::Uuid
}