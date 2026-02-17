/*

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

2) Finding methods using various conditions 

    1) for finding all users :
        Users::find().all(&db).await.unwrap() // returns Option<vec<Users>>
    
    2) for finding user by id:
        Users::find_by_id(id).one(&db).await.unwrap() // returns Option<Users>

    3) for finding Users with multiple conditions: And Operation

        Users::find()
            .filter(Column::Username.eq(username))
            .filter(Column::Password.eq(password))
            .one(&db)
            .await
            .unwrap() // returns Option<Users>

    we can also do filter chaining which is similar to And operation.
    another way to implement And operation

    Users::find()
        .filter(
            Condition::all()
                .add(Column::Username.eq(username))
                .add(Column::Password.eq(password))
        )
        .filter(Column::Password.eq(password))
        .one(&db)
        .await
        .unwrap() // returns Option<Users>

    4) for finding Users with multiple conditions: Or Operation

        Users::find()
            .filter(
                Condition::any()
                    .add(Column::Username.eq(username))
                    .add(Column::Email.eq(username))
            )
            .filter(Column::Password.eq(password))
            .one(&db)
            .await
            .unwrap() // returns Option<Users>

    5) for selecting specific columns from the table

        Users::find()
            .select_only()
            .column(Column::UserId)
            .column(Column::Name)
            .one(&db)
            .await
            .unwrap() // returns Option<(Uuid,String)>  




    Returning Response:

    the return type of the async function should be impl IntoResponse by which we can return Status code and reqest body as response.

    if returning from any condition than it should be in the form of: return keyword is necessary

    return (
        StatusCode,
        Json(Response)
    )
    .into_response();

    and without condition or returning default

    (
        StatusCode,
        Json(Response)
    )
    .into_response();

3) Update operation

    1) first find the record by id and than convert it into ActiveModel and than update the value using Set() and than call the update method by passing the reference of database.

    let user_with_id = Users::find_by_id(user_id)
        .one(&db)
        .await
        .unwrap();

    let mut user:entity::users::ActiveModel = match user_with_id{
        Some(user)=>user.into(),
        None=>{
            return (
                StatusCode::NOT_FOUND,
                Json(
                    ApiResponse{
                        message:"User not found".to_string()
                    }
                )
            ).into_response()
        }

    };

    user.name = Set(update_user_request.name);
    user.email= Set(update_user_request.email);
    user.contact = Set(update_user_request.contact);
    user.username = Set(update_user_request.username);

    let updated_user = user.update(&db).await.unwrap();


4) Delete operation

    1) for deleting record by id

    let user_with_id = Users::find_by_id(user_id)
        .one(&db)
        .await
        .unwrap();

    if user_with_id.is_some(){

        Users::delete_by_id(user_id)
            .exec(&db)
            .await
            .unwrap();

        println!("User deleted successfully");
        db.close().await.unwrap();
        return (
            StatusCode::OK,
            Json(
                ApiResponse{
                    message:"User deleted successfully".to_string()
                }
            )
        ).into_response();

    }else{
        println!("User not found");
        db.close().await.unwrap();

        return (
            StatusCode::NOT_FOUND,
            Json(
                ApiResponse{
                    message:"User not found".to_string()
                }
            )
        ).into_response();
    }

5) finding all records

    Users::find()
        .all(&db)
        .await
        .unwrap(); // returns Vec<Users>



*/


use std::{env};
use axum::{Json,extract::Path,http::StatusCode, response::IntoResponse};
use dotenvy::dotenv;
use entity::users::{self, Column};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, Database, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use crate::users::models::{LoginRequest, UpdateUserRequest, UserCreateRequest};
use entity::prelude::Users;


#[derive(Debug,Serialize,Deserialize)]
pub struct ApiResponse{
    pub message:String
}

pub async fn signup(Json(user_request):Json<UserCreateRequest>)->impl IntoResponse{

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database url not found");
    let db = Database::connect(database_url).await.unwrap();

    let user_with_username = Users::find()
        .filter(Column::Username.eq(&user_request.username))
        .one(&db)
        .await
        .unwrap();

    if user_with_username.is_some(){
        println!("Username already exists");
        return (
            StatusCode::BAD_REQUEST,
            Json(
                ApiResponse{
                    message:"Username already exists".to_string()
                }
            )
        ).into_response();
    }

    let user_with_email = Users::find()
        .filter(Column::Email.eq(&user_request.email))
        .one(&db)
        .await
        .unwrap();

    if user_with_email.is_some(){
        println!("Email already exists");
        return (
            StatusCode::BAD_REQUEST,
            Json(
                ApiResponse{
                    message:"Email already exists".to_string()
                }
            )
        ).into_response();
    }

    let user_with_contact = Users::find()
        .filter(Column::Contact.eq(&user_request.contact))
        .one(&db)
        .await
        .unwrap();

    if user_with_contact.is_some(){
        println!("Contact already exists");

        return (
            StatusCode::BAD_REQUEST,
            Json(
                ApiResponse{
                    message:"Contact already exists".to_string()
                }
            )
        ).into_response()


    }

    let user_model = users::ActiveModel{
        user_id:Set(uuid::Uuid::new_v4().to_owned()),
        name:Set(user_request.name.to_owned()),
        email:Set(user_request.email.to_owned()),
        contact:Set(user_request.contact.to_owned()),
        username:Set(user_request.username.to_owned()),
        password:Set(user_request.password.to_owned()),
        created_at:Set(chrono::Utc::now().naive_utc().to_owned())
    };

    let user_response = user_model.insert(&db).await.unwrap();
    println!("User created successfully: {:?}",user_response);

    db.close().await.unwrap();

    (StatusCode::CREATED,Json(user_response)).into_response()


}

pub async fn login(Json(login_request):Json<LoginRequest>) -> impl IntoResponse{

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database url not found");
    let db = Database::connect(database_url).await.unwrap();

    let user = Users::find()
        .filter(
            Condition::any()
                .add(Column::Username.eq(&login_request.username))
                .add(Column::Email.eq(&login_request.username))
        )
        .filter(Column::Password.eq(login_request.password))
        .one(&db)
        .await
        .unwrap();

        db.close().await.unwrap();

match user {
    Some(user) => {
        println!("User logged in successfully");
        (StatusCode::OK, Json(user)).into_response()
    }
    None => {
        println!("Invalid username or password");
        (
            StatusCode::UNAUTHORIZED,
            Json(
                ApiResponse{
                    message:"Invalid username or password".to_string()
                }
            )
        ).into_response()

    }
}


}


#[axum::debug_handler]
pub async fn update_user(
    Path(user_id):Path<uuid::Uuid>,
    Json(update_user_request):Json<UpdateUserRequest>
)->impl IntoResponse{

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Failed to load database url");
    let db = Database::connect(database_url).await.unwrap();

    let user_with_id = Users::find_by_id(user_id)
        .one(&db)
        .await
        .unwrap();

    let mut user:entity::users::ActiveModel = match user_with_id{
        Some(user)=>user.into(),
        None=>{
            return (
                StatusCode::NOT_FOUND,
                Json(
                    ApiResponse{
                        message:"User not found".to_string()
                    }
                )
            ).into_response()
        }

    };

    user.name = Set(update_user_request.name);
    user.email= Set(update_user_request.email);
    user.contact = Set(update_user_request.contact);
    user.username = Set(update_user_request.username);

    let updated_user = user.update(&db).await.unwrap();
    println!("User updated successfully");

    db.close().await.unwrap();

    (
        StatusCode::OK,
        Json(updated_user)
    ).into_response()

}

pub async fn delete_user(Path(user_id):Path<uuid::Uuid>)->impl IntoResponse{

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database url not found");
    let db = Database::connect(database_url).await.unwrap();

    let user_with_id = Users::find_by_id(user_id)
        .one(&db)
        .await
        .unwrap();

    if user_with_id.is_some(){

        Users::delete_by_id(user_id)
            .exec(&db)
            .await
            .unwrap();

        println!("User deleted successfully");
        db.close().await.unwrap();
        return (
            StatusCode::OK,
            Json(
                ApiResponse{
                    message:"User deleted successfully".to_string()
                }
            )
        ).into_response();

    }else{
        println!("User not found");
        db.close().await.unwrap();

        return (
            StatusCode::NOT_FOUND,
            Json(
                ApiResponse{
                    message:"User not found".to_string()
                }
            )
        ).into_response();
    }

    



}


pub async fn get_all_users()->impl IntoResponse{

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database url not found");
    let db = Database::connect(database_url).await.unwrap();

    let users = Users::find()
        .all(&db)
        .await
        .unwrap();

    db.close().await.unwrap();

    (
        StatusCode::OK,
        Json(users)
    ).into_response()


}

