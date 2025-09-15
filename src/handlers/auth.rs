use axum::{
    extract::State,
    response::Json,
    Extension,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::api::AppState;
use crate::models::{RegisterRequest, LoginRequest, AuthResponse, UserResponse};
use crate::utils::errors::{AppError, AppResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub exp: usize,  // expiration time
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<Json<AuthResponse>> {
    // Check if user already exists
    if state.db.get_user_by_email(&payload.email).await?.is_some() {
        return Err(AppError::Conflict("User already exists".to_string()));
    }

    // Hash password
    let password_hash = hash(&payload.password, DEFAULT_COST)
        .map_err(|_| AppError::InternalServerError("Failed to hash password".to_string()))?;

    // Create user
    let user = state.db.create_user(&payload.email, &password_hash).await?;

    // Generate JWT token
    let token = generate_token(&user.id.to_string(), &state.config.jwt_secret)?;

    Ok(Json(AuthResponse {
        token,
        user: user.into(),
    }))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<Json<AuthResponse>> {
    // Get user by email
    let user = state
        .db
        .get_user_by_email(&payload.email)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

    // Verify password
    let is_valid = verify(&payload.password, &user.password_hash)
        .map_err(|_| AppError::InternalServerError("Failed to verify password".to_string()))?;

    if !is_valid {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    // Generate JWT token
    let token = generate_token(&user.id.to_string(), &state.config.jwt_secret)?;

    Ok(Json(AuthResponse {
        token,
        user: user.into(),
    }))
}

pub async fn me(Extension(user_id): Extension<Uuid>) -> AppResult<Json<UserResponse>> {
    // In a real implementation, you'd fetch the user from the database
    // For now, we'll return basic info from the token
    Ok(Json(UserResponse {
        id: user_id,
        email: "".to_string(), // Would be fetched from DB
        created_at: chrono::Utc::now(), // Would be fetched from DB
    }))
}

fn generate_token(user_id: &str, secret: &str) -> AppResult<String> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| AppError::InternalServerError("Failed to generate token".to_string()))
}

pub fn decode_token(token: &str, secret: &str) -> AppResult<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))
}
