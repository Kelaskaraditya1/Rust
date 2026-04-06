/*
Sftp: Ssh/Secure File transfer protocol 

1) make a sftp client:

    1) create a Sftp client struct.

    pub struct SftpClient{
        pub client:sftp
    }

    2) implementation for struct

    let tcp_connection = Tcp::connect("120.0.0.1.22") // default port of ssf is 22.

    impl SftpClient{

        fn connect()->{

        let mut session = Session::new()
            .set_tcp_stream(tcp_connection)
            .handshake()
        
        session.user_auth_password(username,password)

        Ok(session.sftp())

        }


    }

2) Reading remote file:

    1) opening the file:

    sftp_client.open(&file_path).map_err(Error)?

    2) reading the fille:

    let data = Vec::new();
    sftp_client.read_to_end(&mut data).map_err(Error)?

    but this data is a array of raw bytes. so that has to converted to readable format. 

    3) content = String::from_utf8_lossy(&data)
    this converts into a String format which can be given to CsvReader

    4) Give it to CsvReader

    let reader = CsvReader::new()
        .has_header(true)
        .from_reader(content.as_bytes())

    5) deserialize it in required format.

    let serialized_data:Result<T,csv::Error> = reader.deserialize()
        .collect()

    this gives a response of Result<T,csv::Error> which has to be converted to Result<T,Error(required)>

    let result = serialized_data.map_err(Error).

3) Writing to remote file:

    1) create a writerBuilder object

    let writer = WriterBuilder::new()
        .has_headers(true/false)
        .from_writer(vec![]) // empty buffer for in memory


    2) serialize the data 

    let serialize_data = writer::serialize(data).map_err()

    returns a Result<>

    3) convert the serialize data into a stream of bytes vec[u8]

    let byte_stream = writer.into_inner().map_err()

    returns Result<>

    4) get the sftp client.and create/open the file

    let path = Path::new(file_path)
    let file = sftp::create(path).map_err()

    
    5) write tha byte stream onto the file

    file.write_all(&byte_data).map_err()

    returns Result<>

    6) flush the file.

    flie.flush().map_err()

    returns Result<>






*/

use csv::{ReaderBuilder, WriterBuilder};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::path::Path as NewPath; 
use axum::{Extension, Json, extract::Path, http::StatusCode, response::Result };
use crate::{ exception::global_exception_handler::ApiError, local::service::file_service::ApiResponse, sftp::{configuration::sftp_configuration::SftpClient, service::user_service::{insert_user, read_users}}};

#[allow(dead_code,unused_variables)]

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct User {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub name: String,
    pub contact: String,
    pub email: String,
    pub username: String,
    pub password: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ReadResponse{

    pub status_code:u16,
    pub summary:String,
    pub faild_records:Vec<User>

}

pub async fn read_remote_file(
    Path(file_path): Path<String>,
    Extension(db):Extension<DatabaseConnection>
) -> Result<Json<ReadResponse>, ApiError> {

    /*

    Process:
        1) get the sftp client connection.
        2) open the file using sftp client which returns a file.
        3) now read the file in terms of stream which is vec[u8].
        4) than deserialize the data.
        5) build a readerBuilder object, which has the stream of data
        6) now deserialize the stream data into the native data form but it is in Restult<Model,csv::Error>
        7) map the deserialize data to Result<Model,ApiError>
    
     */
    
    let sftp = SftpClient::connect()
        .await
        .map_err(
            |error| ApiError{
                status_code:StatusCode::INTERNAL_SERVER_ERROR,
                message:format!("Failed to connect to Sftp Server: {}",error.to_string())
            }
        )?.sftp;

    let mut file = sftp
        .open(&file_path)
        .map_err(
            |error| ApiError{
                status_code:StatusCode::INTERNAL_SERVER_ERROR,
                message:format!("Failed to open file: {}",error.to_string())
            }
        )?;

    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .map_err(
            |error| ApiError{
                status_code:StatusCode::INTERNAL_SERVER_ERROR,
                message:format!("Failed to read file: {}",error.to_string())
            }
        )?;

    let content = String::from_utf8_lossy(&data);
    
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)  
        .from_reader(content.as_bytes());

    let users: Result<Vec<User>, csv::Error> = rdr
        .deserialize()
        .collect();

    let users = users.map_err(
        |error| ApiError{
                status_code:StatusCode::BAD_REQUEST,
                message:format!("Failed to map error: {}",error.to_string())
            }
    )?;

    let mut pass = 0;
    let mut fail = 0;
    let mut failed_records = Vec::new();

    for user in users{
        let result = insert_user(Extension(db.clone()),Json(user.clone())).await;
        if result.is_ok(){
            pass+=1;
        }
        else{
            fail+=1;
            failed_records.push(user.clone());
        }
    
    }

    let status = sftp.unlink(NewPath::new(&file_path))
        .map_err(
            |error| println!("failed to delete file:{}",error.to_string())
        );




    let summary = if status.is_ok(){
        format!("Toal records:{} Inserted:{} Failed:{}, and file deleted sucessfully ",(pass+fail),pass,fail)
    }else{
        format!("Toal records:{} Inserted:{} Failed:{}, failed to delete file ",(pass+fail),pass,fail)
    };

    Ok(
        Json(
            ReadResponse{
                status_code:StatusCode::OK.as_u16(),
                summary:summary,
                faild_records:failed_records
            }
        )
    )

}

pub async fn write_to_file(
    Path(file_path):Path<String>,
    Extension(db):Extension<DatabaseConnection>
)->Result<Json<ApiResponse>,ApiError>{

    /*

    1) get data from database.
    2) create writer, serialize the data, convert it into vec[u8] format
    3) establish sftp connection and open the file
    4) write the data onto the file
    5) flush the writer.

     */

    // 1) get the data from database

    let users = read_users(Extension(db)).await
    .map_err(
        |_| ApiError{
            status_code:StatusCode::INTERNAL_SERVER_ERROR,
            message:format!("failed to get users")
        }
    )?;

    // 2) Create WriterBuilder

    let mut writer = WriterBuilder::new()
        .has_headers(true)
        .from_writer(vec![]);

    // 3) serialize the data.

    for user in users{
         writer.serialize(user)
            .map_err(
                |error| ApiError{
                    status_code:StatusCode::INTERNAL_SERVER_ERROR,
                    message:format!("failed to serialize the data:{}",error.to_string())
                }
            )?;
    }

    // 4) convert to bytes format

    let byte_data = writer.into_inner()
        .map_err(
            |_| ApiError{
                status_code:StatusCode::INTERNAL_SERVER_ERROR,
                message:format!("failed to convert data in bytes")
            }
        )?;

    // 5) establish sftp connection

    let sftp = SftpClient::connect()
        .await
        .map_err(
            |error| ApiError{
                status_code:StatusCode::INTERNAL_SERVER_ERROR,
                message:format!("failed to establish connection:{}",error.to_string())
            }
        )?;

        // 6) opening the remote file

        let path = NewPath::new(&file_path);
        let mut file = sftp.sftp.create(path)
            .map_err(
                |error| ApiError{
                    status_code:StatusCode::INTERNAL_SERVER_ERROR,
                    message:format!("failed to open the remote file: {}",error.to_string())
                }
            )?;

        // 7) write the data on the file

         file.write_all(&byte_data)
            .map_err(
                |error| ApiError{
                    status_code:StatusCode::INTERNAL_SERVER_ERROR,
                    message:format!("failed to write data on to file:{}",error.to_string())
                }
            )?;

        file.flush()
            .map_err(
                |error| ApiError{
                    status_code:StatusCode::INTERNAL_SERVER_ERROR,
                    message:format!("failed to flush the writer:{}",error.to_string())
                }
            )?;


        Ok(
            Json(
                ApiResponse{
                    status_code:StatusCode::OK.as_u16(),
                    message:format!("data written to file sucessfully!!")
                }
            )
        )
    

}

/*

Logic:

util:
3 hashmap, contact , email , username
set for duplicate
list for unique user
final list to write


flow:

1) Database 

    1) iterate over object and enter unique username, email and contact in hashmap
    2) if any duplicate enter in set
    3) if not than enter in unique user list

2) File

    1) if all contact, email and username exist in respective hashmaps and all  userId is same for all hashmaps  than enter in final list
    2) if any of it is duplicate than enter in duplicate list.

*/