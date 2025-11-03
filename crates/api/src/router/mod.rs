pub mod auth;

use axum::Router;
use crate::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest("/auth", auth::router())
        .with_state(state)
}