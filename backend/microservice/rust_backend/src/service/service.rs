use axum::{Json, extract::State};
use reqwest::StatusCode;

use crate::{exception::global_exception_handller::ApiError, models::models::{AppState, Users}};

pub async fn get_users(
    State(app_state):State<AppState>
)->Result<Json<Vec<Users>>,ApiError>{

    let url = format!("{}/api/v1/users",app_state.base_url);

    let response = app_state.client
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

        if !status.is_success(){

            let error = response.text()
                .await
                .unwrap_or_else(
                    |_| "No response body from server".to_string()
                );

                return Err(
                    ApiError { status_code: status, message: error }
                );
        }

        let users = response.json::<Vec<Users>>()
            .await
            .map_err(
                |error| ApiError{
                    status_code:StatusCode::INTERNAL_SERVER_ERROR,
                    message:format!("cannot parse response :{}",error)
                }
            )?;

            Ok(Json(users))

}