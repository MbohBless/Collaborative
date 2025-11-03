use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use models::{Claims,Role,User};
use crate::errors::AuthError;

pub struct AuthService {
    jwt_secret: String,
    jwt_expiration_hours: i64,
}

impl AuthService {
    pub fn new(jwt_secret: String, jwt_expiration_hours: i64) -> Self {
        Self {
            jwt_secret,
            jwt_expiration_hours,
        }
    }
    pub fn hash_password(&self, password: &str) -> Result<String, AuthError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hashed_password = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AuthError::PasswordHashing(e.to_string()))
            .map(|hash| hash.to_string())?;
        Ok(hashed_password)
    }
    pub fn verify_password(&self, hashed_password: &str, password: &str) -> Result<bool, AuthError> {
        let parsed_hash = PasswordHash::new(hashed_password)
            .map_err(|e| AuthError::PasswordHashing(e.to_string()))?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())

    }
    pub fn generate_token(&self, user: &User) -> Result<String, AuthError> {
        let now = Utc::now();
        let exp = now + chrono::Duration::hours(self.jwt_expiration_hours);
        let claims = Claims {
            sub: user.id,
            email: user.email.clone(),
            role: user.role.clone(),
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
        };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| AuthError::TokenGeneration(e.to_string()))
    }

    
    pub fn validate_token(&self, token: &str) -> Result<Claims, AuthError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| AuthError::TokenValidation(e.to_string()))?;
        Ok(token_data.claims)
    }
    pub fn check_permission(&self, claims: &Claims, required_role: &Role) -> bool {
        match required_role {
            Role::Admin => matches!(claims.role, Role::Admin),
            Role::Editor => matches!(claims.role, Role::Admin | Role::Editor),
            Role::Viewer => true,
        }
    }
}