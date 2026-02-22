use entity::post::Model;
use entity::users::Model as UserModel;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct PostResponse{

    pub user:UserModel,
    pub posts_list:Vec<Model>

}