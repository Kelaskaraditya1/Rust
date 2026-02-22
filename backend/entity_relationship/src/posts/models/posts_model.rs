use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug,Serialize,Deserialize)]
pub struct Posts{

    pub post_id:Uuid,
    pub title:String,
    pub contnent:String,
    pub media_url:String,
    pub created_at:String,
    pub user_id:Uuid

}