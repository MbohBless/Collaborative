use sqlx::PgPool;
use std::sync::Arc;
use core::AuthService;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub auth_service: Arc<AuthService>,
}

impl AppState {
    pub fn new(db_pool: PgPool, auth_service: AuthService) -> Self {
        Self {
            db_pool,
            auth_service: Arc::new(auth_service),
        }
    }
}
