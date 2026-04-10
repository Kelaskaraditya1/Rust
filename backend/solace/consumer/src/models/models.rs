use reqwest::Client;
use serde::{Deserialize, Serialize};


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
                base_url:"localhost:1000".to_string() 
            }
    }
    
}

#[derive(Debug,Serialize,Deserialize)]
pub struct UserRegisteredEvent {
    pub event_id: String,
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub registered_at: String,
}