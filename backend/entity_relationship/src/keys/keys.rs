use std::env;

use lazy_static::lazy_static;

lazy_static!{

    pub static ref IP_ADDRESS:String= get_ip_address();
    pub static ref DATABASE_URL:String = get_database_url(); 

}

fn get_ip_address()->String{

    dotenvy::dotenv().ok();
    let address = env::var("IP_ADDRESS").expect("unable to find Ip");
    let port = env::var("PORT").expect("unable to find port");

    address + &port
}

fn get_database_url()->String{

    dotenvy::dotenv().ok();
    env::var("DATABASE_URL").expect("unable to find Database url")
}