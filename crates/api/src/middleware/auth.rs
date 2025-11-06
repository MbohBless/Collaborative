use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::{Response},
};
use models::Claims;
use async_trait::async_trait;

use crate::state::AppState;

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req:Request, 
    next: Next,
)-> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());
    
    let auth_header = match auth_header {
        Some(header) => header,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;
    let claims = state.auth_service.validate_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    req.extensions_mut().insert::<Claims>(claims);

    let response = next.run(req).await;
    Ok(response)
}

use axum::extract::FromRequestParts;

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = (StatusCode,&'static str);
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts.extensions.get::<Claims>()
            .cloned()
            .ok_or((StatusCode::UNAUTHORIZED,"No claims found"))
    }

}