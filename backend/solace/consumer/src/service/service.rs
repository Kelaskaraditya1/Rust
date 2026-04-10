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

use axum::{Json, debug_handler, extract::{ State}};
use reqwest::{StatusCode};
use serde::{Deserialize, Serialize};
use crate::{exception::global_exception_handller::ApiError, models::models::{AppState, UserRegisteredEvent, Users}};

use std::{env};

use dotenvy::dotenv;
use lettre::{
    message::header::ContentType,
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

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

#[debug_handler]
pub async fn send_welcome_email(
    Json((name,to_email)):Json<(String,String)>,
) -> Result<(), ApiError> {
    dotenv().ok();

    let smtp_host = env::var("SMTP_HOST").expect("invalid key smpt_host");
    let smtp_username = env::var("SMTP_USERNAME").expect("invalid key smtp username");
    let smtp_password = env::var("SMTP_PASSWORD").expect("invalid key smtp_password");
    let from_email = env::var("FROM_EMAIL").expect("invalid key from_email");
    let from_name = env::var("FROM_NAME").unwrap_or_else(|_| "Support Team".to_string());

    let html_body = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="UTF-8" />
            <title>Welcome</title>
        </head>
        <body style="margin:0;padding:0;background-color:#f4f6f8;font-family:Arial,Helvetica,sans-serif;color:#1f2937;">
            <table role="presentation" width="100%" cellspacing="0" cellpadding="0" style="background-color:#f4f6f8;padding:30px 0;">
                <tr>
                    <td align="center">
                        <table role="presentation" width="600" cellspacing="0" cellpadding="0" style="background-color:#ffffff;border-radius:10px;padding:40px;">
                            <tr>
                                <td>
                                    <h2 style="margin:0 0 20px 0;color:#111827;">Welcome, {name}!</h2>
                                    <p style="margin:0 0 16px 0;font-size:16px;line-height:1.6;">
                                        Thank you for signing up with us. We are delighted to have you on board.
                                    </p>
                                    <p style="margin:0 0 16px 0;font-size:16px;line-height:1.6;">
                                        Your account has been successfully created, and you can now start exploring our platform and its features.
                                    </p>
                                    <p style="margin:0 0 16px 0;font-size:16px;line-height:1.6;">
                                        If you have any questions or need any assistance, our team is always here to help.
                                    </p>
                                    <p style="margin:24px 0 0 0;font-size:16px;line-height:1.6;">
                                        Best regards,<br />
                                        <strong>{from_name}</strong>
                                    </p>
                                </td>
                            </tr>
                        </table>
                    </td>
                </tr>
            </table>
        </body>
        </html>
        "#
    );

    let email = Message::builder()
        .from(format!("{from_name} <{from_email}>").parse().expect("failed to parse from"))
        .to(to_email.parse().expect("failed to parse to_mail"))
        .subject("Welcome to our platform")
        .header(ContentType::TEXT_HTML)
        .body(html_body).map_err(
            |error| ApiError{
                status_code:StatusCode::INTERNAL_SERVER_ERROR,
                message:format!("error in email body: {}",error.to_string())
            }
        )?;

    let creds = Credentials::new(smtp_username, smtp_password);

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_host).expect("unable to parse mailer")
        .credentials(creds)
        .build();

    mailer.send(email).await.map_err(
        |error| ApiError{
            status_code:StatusCode::INTERNAL_SERVER_ERROR,
            message:format!("unable to send email:{}",error.to_string())
        }
    )?;

    Ok(())
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ApiResponse{
    pub status_code:u16,
    message:String
}

#[debug_handler]
pub async fn get_consumer_data(
    Json(payload): Json<UserRegisteredEvent>
)->Result<Json<ApiResponse>,ApiError>{

    println!("event id: {}",payload.event_id.clone());
    println!("email: {}",payload.email.clone());
    println!("name: {}",payload.name.clone());

    let status = send_welcome_email(Json((payload.name.clone(),payload.email.clone()))).await;

    if status.is_ok(){
        return Ok(
            Json(
                ApiResponse { status_code: StatusCode::OK.as_u16(), message: format!("email send to email {} successfully",payload.email.clone()) }
            )
        );
    }else{
        Err(
            ApiError { status_code: StatusCode::INTERNAL_SERVER_ERROR, message: "failed to send email".to_string() }
        )
    }



}