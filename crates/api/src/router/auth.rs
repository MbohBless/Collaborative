use axum::{
    extract::State,
    routing::{post},
    Json, Router,
};
use db::repositories::UserRepository;
use models::{AuthResponse, CreateUser, LoginRequest, RegisterRequest, UserResponse};

use crate::{errors::ApiError, state::AppState};

pub fn router() -> Router<AppState>{
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}

async fn register(
    State(state):State<AppState>,
    Json(payload):Json<RegisterRequest>,
)->Result<Json<AuthResponse>, ApiError>{
    if payload.email.is_empty() || payload.password.is_empty() || payload.username.is_empty() {
        return Err(ApiError::Internal("Email, username, and password must be provided".into()));
    }
    let user_repo = UserRepository::new(state.db_pool.clone());
    if user_repo.find_by_email(&payload.email).await?.is_some() {
        return Err(core::AuthError::UserAlreadyExists.into());
    }

    let password_hash = state.auth_service.hash_password(&payload.password)?;
    let new_user = CreateUser {
        email: payload.email,
        username: payload.username, 
        password: payload.password,
        role: models::Role::Viewer,
    };
    let user = user_repo.create(new_user, password_hash).await?;
    let token = state.auth_service.generate_token(&user)?;
    let response = AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            email: user.email,
            username: user.username,
            role: user.role,
        },
    };

    Ok(Json(response))
}

async fn login(
    State(state):State<AppState>,
    Json(payload):Json<LoginRequest>,
)->Result<Json<AuthResponse>, ApiError>{
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(ApiError::Internal("Email and password must be provided".into()));
    }
    let user_repo = UserRepository::new(state.db_pool.clone());
    let user = match user_repo.find_by_email(&payload.email).await? {
        Some(user) => user,
        None => return Err(core::AuthError::InvalidCredentials.into()),
    };

    let is_valid = state.auth_service.verify_password(&user.password_hash, &payload.password)?;
    if !is_valid {
        return Err(core::AuthError::InvalidCredentials.into());
    }

    let token = state.auth_service.generate_token(&user)?;
    let response = AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            email: user.email,
            username: user.username,
            role: user.role,
        },
    };

    Ok(Json(response))
}
