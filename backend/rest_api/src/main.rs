/*
1) Dependencies installed:

 1) cargo add axum --features=http2,macros,ws
 2) cargo add tokio --features=full
 3) cargo add serde --features=derive
 4) cargo add uuid --features=v4
 5) cargo add chrono // for date_time.

 Sea-Orm-cli

 sea-orm-cli is used for creating tables and migration 

 1) cargo install sea-orm-cli to install sea-orm-cli
 2) run, sea-orm-cli migrate init // creates migration folder in which there is file where we have to update the schema of the table.
 3) run, sea-orm-cli migrate fresh // runs the migration and creates table in database


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

than import , use entity::<entity>;

Crud Operations:

1) Insert , for taking Response body as input: Json(payload):Json<Type> and the return type should be impl IntoResponse

steps for insert operation:
    1) use the ActiveModel from the entity that imported from the entity dir and Set() to set the value 

    let user_model = user::ActiveModel{
    user_id:Set(uuid::Uuid::new_v4()),
    name:Set(user_request.name),
    created_at:Set(chrono::Utc::now(),naive_utc()
    }

    2) than use user-model to call the insert method and pass the reference of Database as a parameter.

    let user_response = user_model.insert(&db).await.unwrap(); this returns the model which is inserted in the database.

    3) return the response with the status code

    (StatusCode::CREATED,Json(user_response))








 */

// use std::env;
// use dotenvy::dotenv;   
use axum::{Json, Router, routing::{get, post}};
// use sea_orm::{Database, DatabaseConnection};
use serde::{Deserialize, Serialize};

pub mod users;

#[tokio::main]
async fn main(){

    let app = Router::new()
        .route("/health",get(health))
        .route("/users/create",post(users::create_user));

    let address = "0.0.0.0:8000";
    let listner = tokio::net::TcpListener::bind(address)
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

// async fn get_database_connection()->DatabaseConnection{

//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL").expect("Database url not found");
//     let database_connection:DatabaseConnection = Database::connect(database_url).await.unwrap();
//     database_connection

// }

async fn health()->Json<Health>{
    Json(
        Health {
            status:"ok".to_string(),
            health:"healthy".to_string()
        }
    )
}

