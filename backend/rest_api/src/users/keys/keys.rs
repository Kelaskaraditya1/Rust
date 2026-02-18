use std::env;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref DATABASE_URL:String = set_database_url();
    pub static ref ADDRESS:String = get_address();
    pub static ref JWT_SECRET:String = get_jwt_secret();
}

fn set_database_url()->String{

    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("unable to find database url");
    database_url

}

fn get_address()->String{
    let address = env::var("IP_ADDRESS").expect("Unable to find Ip address");
    let port = env::var("PORT").expect("unable to get port");

    address + &port
}

fn get_jwt_secret()->String{
    let jwt_secret = env::var("JWT_SECRET").expect("unable to get Jwt Secret");
    jwt_secret
}