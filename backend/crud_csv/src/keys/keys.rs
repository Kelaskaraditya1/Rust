use std::env;

use lazy_static::lazy_static;

lazy_static!{

    pub static ref ADDRESS:String=get_address();
    pub static ref DATABASE_URL:String = get_database_url();
    pub static ref SFTP_USERNAME:String= get_sftp_username();
    pub static ref SFTP_PASSWORD:String = get_sftp_password();
    pub static ref TCP_ADDRESS:String = get_tcp_address();

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

fn get_sftp_username()->String{

    dotenvy::dotenv().ok();
    env::var("SFTP_USERNAME").expect("Invalid key SFTP_USERNAME")

}

fn get_sftp_password()->String{

    dotenvy::dotenv().ok();
    env::var("SFTP_PASSWORD").expect("Invalid key SFTP_PASSWORD")

}

fn get_tcp_address()->String{
    dotenvy::dotenv().ok();
    env::var("TCP_ADDRESS").expect("Invalid key TCP_ADDRESS")
}
