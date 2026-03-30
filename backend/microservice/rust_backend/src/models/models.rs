use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::keys;

#[derive(Debug,Serialize,Deserialize,Clone)]
#[serde(rename_all = "camelCase")]
pub struct Users{

    user_id:String,
    name:String,
    email:String,
    contact:String,
    username:String,
    password:String

}

#[derive(Debug,Clone)]
pub struct AppState{
    pub client:Client,
    pub base_url:String
}

impl AppState {

    pub fn new()->Self{

        Self { client: Client::new(),
                base_url: keys::keys::BASE_URL.to_string() 
            }
    }
    
}