use std::env;

use lazy_static::lazy_static;

lazy_static!{

    pub static ref ADDRESS:String=get_address();
    pub static ref DATABASE_URL:String = get_database_url();

}

fn get_address()->String{
    dotenvy::dotenv().ok();

    let ip = env::var("IP").expect("Invalid key IP");
    let port = env::var("PORT").expect("Invalid key PORT");

    ip + &port
}

fn get_database_url()->String{

    dotenvy::dotenv().ok();
    env::var("DATABASE_URL").expect("Invalid key DATABASE_URL")

}