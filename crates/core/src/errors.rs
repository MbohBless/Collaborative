use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    #[error("User already exists")]
    UserAlreadyExists,
    
    #[error("Token generation failed: {0}")]
    TokenGeneration(String),
    
    #[error("Token validation failed: {0}")]
    TokenValidation(String),
    
    #[error("Password hashing failed: {0}")]
    PasswordHashing(String),
    
    #[error("Database error: {0}")]
    Database(#[from] db::DbError),
}

