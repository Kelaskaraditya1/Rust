/* File Handeling Notes:

1) we pass file in form of Multipart and we can send a set of files in this request.
so when we take it as a parameter in a function we have to loop over the parameter and check whether there is a next field or not.

for eg: multipart_file.next_field().await.map_err()?;

2) For getting the file name from the field/file,
field.file_name() 
this returns Option<>

3) For getting the data in bytes from filed of multipart:

    field.bytes().await{
    Ok(byte),
    Err(err)
    }

4) For creating a new file:
    we require the path where the file has to be created and the name of the file

    File::create(file_path)
    returns Option<File,None>

5) For Opening a file:

    File::open(file_path) gives a Result<file>

6) For writing data on to the file : we require a mutable reference of file
    data should be in &[u8]
    file::write_all(data)
    this returns a Result<(),Error> so have to use "if let" to handle the error

7) For Reading data from file:

    1) we require 2 things, instance of file and file reader

    file_reader = FileReader::new()
        .from_headers(true/false) -> depends whether the file has the column name or not
        .from_reader(file);

    for (index,record) in file_reader.records().enumerated{

        here the record is of Result<> so therefor

        match record{
        Ok(value)=>{},
        Err(err)=>{}
        } ;

    }

    8) For deleting the file:

    std::fs:remove_file(path) gives Result

    9) for identifiying the type of the file: we have used a 3rd party crate for it that is infer
    we require data in bytes.

    match infer::get(byte-data){ // gives Optionl<Type,Error>

    Some(value)=>{

            let mime = value.mime_type()

            // compare mime with required format.
        }
    } 

    10) for writing data on to a csv file we have to use the csv crate

    first create a Csv Writer, write the data using serialize() than flush the writer. 

    let writer = Writer::from_path(path) gives Result<>
    writer.serialize(obj) returns Result<>
    writer.flush() returns Result<>

*/

use axum::{Extension, Json, debug_handler, extract::{Multipart}, http::StatusCode};
use csv::{ReaderBuilder, Writer};
use entity::users::Model;
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use serde::Serialize;
use crate::{local::dto::request_dto::UserRequestDto, exception::global_exception_handler::ApiError, local::service::user_service::insert_user};
use std::{env, fs::{self, File}, io::Write};


#[derive(Debug,Serialize)]
pub struct ApiResponse{
    pub status_code:u16,
    pub message:String
}

pub async fn save_file(file_name:&str, data:&[u8])->Result<String,ApiError>{

    let file_path = format!("uploads/{}", file_name);

    let mut file = match File::create(&file_path) {
        Ok(f) => f,
        Err(e) => {
            return Err(ApiError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Error creating file '{}': {}", file_name, e)
            });
        }
    };

    if let Err(e) = file.write_all(data) {
        return Err(ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("Failed to write data to '{}': {}", file_name, e)
        });
    }

    Ok(file_path)
}

async fn read_file(file_path: &str, db: &DatabaseConnection) -> Result<ApiResponse, ApiError> {

    let mut success_count: u32 = 0;
    let mut fail_count: u32 = 0;

    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            return Err(
                ApiError { status_code: StatusCode::INTERNAL_SERVER_ERROR, message: format!("Unable to open file '{}': {}", file_path, e) }
            );
        }
    };

    

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    for (row_index, result) in reader.records().enumerate() {
        let row_num = row_index + 2; 

        let record = match result {
            Ok(r) => r,
            Err(e) => {
                eprintln!("[Row {}] CSV parse error: {}", row_num, e);
                fail_count += 1;
                continue;
            }
        };

        let user_request = UserRequestDto {
            user_id: record.get(0).unwrap_or_default().to_string(),
            name: record.get(1).unwrap_or_default().to_string(),
            contact: record.get(2).unwrap_or_default().to_string(),
            email: record.get(3).unwrap_or_default().to_string(),
            username: record.get(4).unwrap_or_default().to_string(),
            password: record.get(5).unwrap_or_default().to_string(),
            created_at: record.get(6).unwrap_or_default().to_string(),
        };

        let user_id = user_request.user_id.clone();

        match insert_user(db, user_request).await {
            Ok(_) => {
                success_count += 1;
                println!("[Row {}] Inserted user '{}' successfully", row_num, user_id);
            }
            Err(e) => {
                fail_count += 1;
                eprintln!("[Row {}] Failed to insert user '{}': {}", row_num, user_id, e.message);
            }
        }
    }

    let summary = format!(
        "CSV processing complete: {} inserted, {} failed (total {} rows)",
        success_count, fail_count, success_count + fail_count
    ); 
    println!("{}", summary);

    if success_count == 0 && fail_count > 0 {
        return Err(ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: summary,
        });
    }

    if  fail_count==0{
        match fs::remove_file(file_path){
            Ok(_)=> println!("File '{}' deleted successfully", file_path),
            Err(error) =>{
                println!("Failed to delete file: {}",error);
            }
        }
    }

    Ok(ApiResponse {
        status_code: StatusCode::OK.as_u16(),
        message: summary,
    })
}



#[debug_handler]
pub async fn upload_file(
    Extension(db): Extension<DatabaseConnection>,
    mut multipart_file: Multipart,
) -> Result<Json<ApiResponse>, ApiError> {
    
    while let Some(field) = multipart_file.next_field().await.map_err(|e| ApiError {
        status_code: StatusCode::BAD_REQUEST,
        message: format!("Failed to read multipart field: {}", e),
    })? {

        let file_name = match field.file_name() {
            Some(name) => name.to_string(),
            None => continue, 
        };
        println!("file name: {}", file_name);

        let data = match field.bytes().await {

            Ok(byte) => byte,
            Err(e) => {
                return Err(
                 ApiError{
                    status_code:StatusCode::INTERNAL_SERVER_ERROR,
                    message: format!("Failed to read file bytes: {}", e),
                    }
                );
            }
            
        };

        let kind = infer::get(&data);

        let is_valid = match kind {

            Some(k) => {
                let mime = k.mime_type();

                    mime == "text/csv"
                    || mime == "application/vnd.ms-excel"
                    || mime == "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
            },
            None => {
                file_name.ends_with(".csv")
            }
            
        };


        if !is_valid{
            return Err(
                ApiError{
                    status_code:StatusCode::BAD_REQUEST,
                    message:"Only csv or excel files are allowed".to_string()
                }
            );
        }

        match save_file(&file_name, &data).await{
            Ok(path) => {
                println!("{file_name} saved successfully");
                let response = read_file(&path, &db).await?;
                return Ok(Json(response));
            },
            Err(e) => {
                return Err(e);
            }
            
        };
    }

    Err(
        ApiError { 
            status_code: StatusCode::BAD_REQUEST,
             message: "No file found in request".to_string() 
            }
    )

}

async fn get_data(
)->Result<Vec<Model>,ApiError>{

    dotenvy::dotenv().ok();
    let db = Database::connect(env::var("DATABASE_URL").expect("Invalid key DATABASE_URL")).await.unwrap();

    let data = entity::prelude::Users::find()
        .all(&db)
        .await
        .map_err(
            |error| ApiError{
                status_code:StatusCode::INTERNAL_SERVER_ERROR,
                message:error.to_string()
            }
        )?;

        Ok(data)
}

#[debug_handler]
pub async fn write_file()->Result<Json<ApiResponse>,ApiError>{

    let response = match get_data().await{
        Ok(value)=>value,
        Err(_)=>{
            return Err(
                ApiError{
                    status_code:StatusCode::INTERNAL_SERVER_ERROR,
                    message:"failed to write data to new file".to_string()
                }
            );
        }
    };

    let path = "uploads/temp_write.csv".to_string();

    match File::create(&path) {
        Ok(_)=>{

            let mut writer = Writer::from_path(path).map_err(
                |error| ApiError{
                    status_code:StatusCode::INTERNAL_SERVER_ERROR,
                    message:format!("error creating the writer {}",error)
                }
            )?;

            for user in response.iter(){

                let _ = writer.serialize(user).map_err(
                    |error| ApiError{
                        status_code:StatusCode::INTERNAL_SERVER_ERROR,
                        message:format!("error while writing record:{}",error)
                    }
                );

            }

            let _ = writer.flush().map_err(
                |error | ApiError{
                    status_code:StatusCode::INTERNAL_SERVER_ERROR,
                    message:format!("Error while flushing the csv writer:{}",error)
                }
            );


            return Ok(Json(
                ApiResponse{
                    status_code:StatusCode::OK.as_u16(),
                    message:"File created sucessfully".to_string()
                }
            ));
        },
        Err(_)=>{
            return Err(
                ApiError { status_code: StatusCode::INTERNAL_SERVER_ERROR, message:"failed to create new file".to_string() }
            )
        }   
    };



}
