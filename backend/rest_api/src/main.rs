/*

<----------------------------------------------------------------------Dependencies to be Installed----------------------------------------------------------------------------------------------------------------------------------------------->

1) Dependencies installed:

 1) cargo add axum --features=http2,macros,ws
 2) cargo add tokio --features=full
 3) cargo add serde --features=derive
 4) cargo add uuid --features=v4
 5) cargo add chrono // for date_time.
 6) cargo add tower-http 
 7) cargo add tower-http -features=cors
 8) cargo add sea-orm-cli
 9) 
<-------------------------------------------------------------------------Sea Orm Cli commands---------------------------------------------------------------------------------------------------------------------------------------------->
 Sea-Orm-cli

 sea-orm-cli is used for creating tables and migration 

 1) cargo install sea-orm-cli to install sea-orm-cli
 2) run, sea-orm-cli migrate init // creates migration folder in which there is file where we have to update the schema of the table.
 3) run, sea-orm-cli migrate fresh // runs the migration and creates table in database

<-------------------------------------------------------------------------Methods for defining schema in migration file---------------------------------------------------------------------------------------------------------------------------------------------->

common methods for updating the schema of the table in the migration file:

1) without ColumDefinition

    1) uuid(User::UserId).not_null().primary_key()    // for craeting a primary of type uuid
    2) big_integer(Users::UserId).not_null().auto_increment().primary_key()    // for creating a primary of type big integer with auto increment  
    3) string(User::Name).not_null()    // for creating a column of type string which is not null
    4) string_uniq(User::Email).not_null()    // for creating a column of type string which is not null and unique
    5) string_uniq(User::Username).not_null()
    6) string(User::Password).not_null())
    7) string_uniq(User::Uuid).not_null())
    8) timestamp(User::CreatedAt).not_null().default(Expr::current_timestamp())    // for creating a column of type timestamp which is not null and has default value as current timestamp

2) with ColumnDefinition

    1) ColumnDef::new(User::Id).big_integer().not_null().auto_increment().primary_key()    // for craeting a primary of type Long
    2) ColumDef::new(User::UserId).uuid().not_null().primary_key()
    3) ColumnDef::new(User::Name).string().not_null()    // for creating a primary of type big integer with auto increment
    4) ColumnDef::new(User::Email).string().unique_key().not_null()    // for creating a column of type string which is not null
    5) ColumnDef::new(User::Username).string().unique_key().not_null()    // for creating a column of type string which is not null and unique
    6) ColumnDef::new(User::Password).string().not_null()
    7) ColumnDef::new(User::Uuid).string().unique_key().not_null()
    8) ColumnDef::new(User::CreatedAt).timestamp().default(Expr::current_timestamp()).not_null()    // for creating a column of type timestamp which is not null and has default value as current timestamp

after creating proper tables in database , next step is to crteate entity dir

for that first run the below command
4) sea-orm-cli generate entity -u postgresql://postgres:9819375722Aditya@localhost:5432/test -o entity/src -l --with-serde both --expanded-format

than create a Cargo.toml file in the entity dir and add the below code in it.

[package]
name = "entity"
version = "0.1.0"
edition = "2021"
publish = false

[lib]
name = "entity"
path = "src/lib.rs"

[dependencies]
sea-orm.workspace = true
serde.workspace = true

than modify the Cargo.toml file of the root dir accordingly the Cargo.toml of entity.

<-------------------------------------------------------------------------For creating Dependency Injection---------------------------------------------------------------------------------------------------------------------------------------------->

for defining constants: 
    1) install lazy_static : cargo add lazy_static
    2) create a file keys.rs

    lazy_static!{
    pub static ref DATABASE_URL:String = set_database_url();
}

    now create a database connection using Database url 

        dotenvy::dotenv().ok(); // loads the .env file
    let database_url = (*keys::keys::DATABASE_URL).clone();
    let database_connection = Database::connect(database_url).await.expect("Failed to connect to Database");

    let app = Router::new()
        .route("/api/v1/health",get(health))
        .merge(router())
        .layer(Extension(database_connection));


    now for using wher ever it is required:

    pub async fn signup(
    Extension(db):Extension<DatabaseConnection>, // taking as a input
    )->impl IntoResponse{

    ...
    }
 
 */

use axum::{Extension, Json, Router, routing::get};
use sea_orm::Database;
use serde::{Deserialize, Serialize};
use crate::users::{keys, routes::routes::router};

pub mod users;

#[tokio::main]
async fn main(){

    dotenvy::dotenv().ok(); // loads the .env file
    let database_url = (*keys::keys::DATABASE_URL).clone();
    let database_connection = Database::connect(database_url).await.expect("Failed to connect to Database");

    let app = Router::new()
        .route("/api/v1/health",get(health))
        .merge(router())
        .layer(Extension(database_connection)); // Creting a dependency injection of Database connection

    let ip_address = (*keys::keys::ADDRESS).clone();

    let listner = tokio::net::TcpListener::bind(ip_address)
        .await
        .unwrap();

    axum::serve(listner,app).await
        .unwrap();
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Health{
    status:String,
    health:String
}

async fn health()->Json<Health>{
    Json(
        Health {
            status:"ok".to_string(),
            health:"healthy".to_string()
        }
    )
}

