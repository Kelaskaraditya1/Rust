use std::env;
use lazy_static::lazy_static;

lazy_static!{

    pub static ref IP_ADDRESS:String = get_ip_address();
    // pub static ref BASE_URL:String = get_base_url();

}

pub fn get_ip_address()->String{
        dotenvy::dotenv().ok();

    let address = env::var("ADDRESS").expect("invalid key ADDRESS");
    let port = env::var("PORT").expect("invalid key PORT");

    address + &port
}

// pub fn get_base_url()->String{
//     dotenvy::dotenv().ok();

//     env::var("JAVA_BASE_URL").expect("invalid key JAVA_BASE_URL")
// }