use std::env;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref DATABASE_URL:String = set_database_url();
}

fn set_database_url()->String{

    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("unable to find database url");
    database_url

}

