
use axum::{Extension, Json, debug_handler, extract::Multipart, http::StatusCode};
use csv::ReaderBuilder;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use crate::{dto::request_dto::UserRequestDto, exception::global_exception_handler::ApiError, service::user_service::insert_user};
use std::{fs::File, io::Write};


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
