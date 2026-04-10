/*
    Mqtt: it is a protocol like http for sending request , the issue with http is that we it is sunchronous , it waits for respopnse.
        Mqtt is asynchronous it doesnot waits for the response, it fires the request and continues the execution ahead.
*/

// use std::{env};
use reqwest::Client;

use crate::keys;

#[derive(Debug,Clone)]
pub struct AppState{
    pub client:Client,
    pub base_url:String
}

impl AppState{
    pub fn new()->Self{
        Self { 
            client:Client::new(),
            base_url:keys::keys::SOLACE_BASE_URL.to_string()
         }
    }
}



// pub async fn solus_client()->AsyncClient{

//     dotenvy::dotenv().ok();

//     let id = env::var("MQTT_USERNAME").expect("invalid key MQTT_USERNAME");
//     let host = env::var("MQTT_HOST").expect("invalid key MQTT_HOST");
//     let port:u16 = env::var("MQTT_PORT").expect("invalid key MQTT_PORT").parse().unwrap();

//     // create a Mqtt Connection.
//     let mut mqtt_options = MqttOptions::new(id, host, port);
//     mqtt_options.set_credentials("default".to_string(), "default".to_string());

//     // creating a Mqtt Client
//     let (client, mut eventloop) = AsyncClient::new(mqtt_options,10);

//     // Runs a background task to keep the connection alive.
//     tokio::spawn(async move {
//         while let Ok(event) = eventloop.poll().await {
//             // Handle incoming messages
//             println!("MQTT event: {:?}", event);
//         }
//     });

//     client

// }