use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
    body::Body,
};

use crate::users::service::jwt_service::decode_jwt;

pub async fn guard(
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {

    // Extract the Authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Check for "Bearer " prefix and extract the token
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Decode and validate the JWT
    let token_data = decode_jwt(token.to_string())?;

    // Store the user's email (from `sub` claim) in request extensions
    // so downstream handlers can access the authenticated user
    request.extensions_mut().insert(token_data.claims);

    Ok(next.run(request).await)
}