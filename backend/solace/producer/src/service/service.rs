use axum::{ Json, debug_handler, http::StatusCode, response::Result};
use reqwest::{Client, };
use uuid::Uuid;
use crate::{ exception::global_exception_handler::ApiError, keys, model::models::{UserRegisteredEvent, Users, UsersRequest}};

#[debug_handler]
pub async fn signup(
    Json(user_request):Json<UsersRequest>
)->Result<Json<Users>,ApiError>{

    // Rest request.

    let user = Users{
        user_id:uuid::Uuid::new_v4().to_string(),
        name:user_request.name.to_string(),
        email:user_request.email.to_string(),
        contac:user_request.contact.to_string(),
        username:user_request.username.to_string(),
        password:user_request.password.to_string(),
        created_at:chrono::Utc::now().naive_utc()
    };

    let user_register_event = UserRegisteredEvent{
        event_id:Uuid::new_v4().to_string(),
        user_id:user.user_id.clone(),
        email:user.email.clone(),
        name:user.name.clone(),
        registered_at:user.created_at.to_string()
    };

    let status = publish_user_registered_event(&Client::new(),&keys::keys::SOLACE_BASE_URL,&user_register_event).await;

    if status.is_ok(){
        return Ok(
            Json(user)
            );
    }else {
        return Err(
            ApiError{
                status_code:StatusCode::INTERNAL_SERVER_ERROR,
                message:format!("failed to enter data to solace")
            }
        );
    }


}

pub async fn publish_user_registered_event(
    client: &reqwest::Client,
    base_url: &str,
    user_registered_event: &UserRegisteredEvent,
) -> Result<(), ApiError> {
    let url = format!("{}/TOPIC/auth/user/registered", base_url);

    let response = client 
        .post(&url)
        .basic_auth("default", Some("default"))
        .header("Solace-Destination", "auth/user/registered")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(user_registered_event)
        .send()
        .await
        .map_err(|error| {
            println!("failed to call solace at url:{} and the error is: {:?}", url, error);
            ApiError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("failed to call solace at url:{} and the error is: {:?}", url, error),
            }
        })?;

    let status = response.status();
    println!("Status: {}", status);

    if !status.is_success() {
        let error_message = response.text().await.unwrap_or_default();

        return Err(ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!(
                "status is not successful:{} and the error message is {}",
                status, error_message
            ),
        });
    }

    Ok(())
}