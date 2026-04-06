/* Notes:

For making the backend a HttpClient we have reqwest crate
command: cargo add reqwest --features=json

than create AppState for which the api request has to be made and craete it's state so that it will follow a singleton pattern.

AppState should innvolve the baseurl and the Client.

let response = Client::new()  // triggers Api Call.
    .get(&url)
    .send()
    .await
    .map_err()?;

let status = response.status();

if !status.is_sucessful(){  // checking the status

let error = response.text().await.unwrap();

}

let users= response.json::Vec<Users>()  // deserializing the response into required format.
    .await
    .map_err()

*/

use axum::{Json, extract::State};
use reqwest::{StatusCode};
use crate::{exception::global_exception_handller::ApiError, models::models::{AppState, Users}};

pub async fn get_users(
    State(app_state):State<AppState>
)->Result<Json<Vec<Users>>,ApiError>{

    let url = format!("{}/api/v1/users",app_state.base_url);

    let response = app_state.client  // Sending the request.
        .get(&url)
        .send()
        .await
        .map_err(
            |error| ApiError{
                status_code:StatusCode::INTERNAL_SERVER_ERROR,
                message:format!("failed to connect to backend server: {}",error)
            }
        )?;

        let  status = response.status();

        if !status.is_success(){ // if request is not sucessful

            let error = response.text()
                .await
                .unwrap();

                return Err(
                    ApiError { status_code: status, message: error }
                );
        }

        let users = response.json::<Vec<Users>>() // Deserializing the response , passing generic type 
            .await
            .map_err(
                |error| ApiError{
                    status_code:StatusCode::INTERNAL_SERVER_ERROR,
                    message:format!("cannot parse response :{}",error)
                }
            )?;

            Ok(Json(users))

}