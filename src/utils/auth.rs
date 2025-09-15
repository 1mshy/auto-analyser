use axum::{
    extract::Request,
    http::{header::AUTHORIZATION, HeaderValue, StatusCode},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::handlers::auth::decode_token;
use crate::utils::errors::AppError;

pub async fn auth_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|auth_str| {
            if auth_str.starts_with("Bearer ") {
                Some(auth_str[7..].to_owned())
            } else {
                None
            }
        });

    let token = auth_header.ok_or_else(|| {
        AppError::Unauthorized("Missing or invalid authorization header".to_string())
    })?;

    // For this example, we'll use a dummy secret. In production, get this from config.
    let secret = "your-secret-key-change-in-production";
    let claims = decode_token(&token, secret)?;

    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?;

    request.extensions_mut().insert(user_id);

    Ok(next.run(request).await)
}
