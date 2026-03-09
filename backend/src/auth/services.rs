use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

use crate::shared::types::Claims;
use crate::shared::AppError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| AppError::Internal(format!("Password hashing failed: {}", e)))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| AppError::Internal(format!("Invalid password hash: {}", e)))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn generate_token(
    user_id: Uuid,
    username: &str,
    roles: Vec<String>,
    permissions: Vec<String>,
    secret: &str,
    expiry_minutes: i64,
) -> Result<String, AppError> {
    let now = Utc::now();
    let claims = Claims {
        sub: user_id,
        username: username.to_string(),
        roles,
        permissions,
        exp: (now + chrono::Duration::minutes(expiry_minutes)).timestamp() as usize,
        iat: now.timestamp() as usize,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("Token generation failed: {}", e)))
}

/// Validate password strength: min 8 chars, at least 1 uppercase, 1 lowercase, 1 digit.
pub fn validate_password_strength(password: &str) -> Result<(), AppError> {
    if password.len() < 8 {
        return Err(AppError::Validation(
            "Password must be at least 8 characters".to_string(),
        ));
    }
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(AppError::Validation(
            "Password must contain at least one uppercase letter".to_string(),
        ));
    }
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(AppError::Validation(
            "Password must contain at least one lowercase letter".to_string(),
        ));
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(AppError::Validation(
            "Password must contain at least one digit".to_string(),
        ));
    }
    Ok(())
}

/// Generate a cryptographically random refresh token (64 hex characters).
pub fn generate_refresh_token() -> String {
    use rand::Rng;
    let bytes: Vec<u8> = (0..32).map(|_| rand::thread_rng().gen::<u8>()).collect();
    hex::encode(bytes)
}

/// Hash a refresh token using SHA-256 for secure storage.
pub fn hash_refresh_token(token: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

pub fn validate_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_hash_and_verify() {
        let password = "test_password_123";
        let hash = hash_password(password).unwrap();
        assert!(verify_password(password, &hash).unwrap());
    }

    #[test]
    fn wrong_password_fails_verification() {
        let hash = hash_password("correct_password").unwrap();
        assert!(!verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn generate_and_validate_token() {
        let user_id = Uuid::new_v4();
        let secret = "test-secret-key";
        let token = generate_token(
            user_id,
            "testuser",
            vec!["USER".to_string()],
            vec!["fi:read".to_string()],
            secret,
            1,
        )
        .unwrap();
        let claims = validate_token(&token, secret).unwrap();
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.username, "testuser");
        assert_eq!(claims.roles, vec!["USER"]);
        assert_eq!(claims.permissions, vec!["fi:read"]);
    }

    #[test]
    fn invalid_token_rejected() {
        let result = validate_token("garbage.token.value", "secret");
        assert!(result.is_err());
    }

    #[test]
    fn password_strength_valid() {
        assert!(validate_password_strength("Abcdef1x").is_ok());
        assert!(validate_password_strength("MyP@ss99").is_ok());
    }

    #[test]
    fn password_strength_too_short() {
        assert!(validate_password_strength("Ab1").is_err());
    }

    #[test]
    fn password_strength_no_uppercase() {
        assert!(validate_password_strength("abcdefg1").is_err());
    }

    #[test]
    fn password_strength_no_lowercase() {
        assert!(validate_password_strength("ABCDEFG1").is_err());
    }

    #[test]
    fn password_strength_no_digit() {
        assert!(validate_password_strength("Abcdefgh").is_err());
    }

    #[test]
    fn refresh_token_generation() {
        let token = generate_refresh_token();
        assert_eq!(token.len(), 64);
        // Should be hex
        assert!(token.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn refresh_token_hashing() {
        let token = "test_refresh_token";
        let hash1 = hash_refresh_token(token);
        let hash2 = hash_refresh_token(token);
        assert_eq!(hash1, hash2); // Deterministic
        assert_ne!(hash1, token); // Different from input
    }

    #[test]
    fn refresh_token_uniqueness() {
        let t1 = generate_refresh_token();
        let t2 = generate_refresh_token();
        assert_ne!(t1, t2);
    }

    #[test]
    fn different_tokens_different_hashes() {
        let h1 = hash_refresh_token("token_a");
        let h2 = hash_refresh_token("token_b");
        assert_ne!(h1, h2);
    }

    #[test]
    fn token_wrong_secret_rejected() {
        let user_id = Uuid::new_v4();
        let token = generate_token(user_id, "user1", vec![], vec![], "secret_one", 60).unwrap();
        assert!(validate_token(&token, "secret_two").is_err());
    }

    #[test]
    fn token_preserves_multiple_roles_and_permissions() {
        let user_id = Uuid::new_v4();
        let roles = vec!["ADMIN".to_string(), "USER".to_string()];
        let perms = vec![
            "fi:read".to_string(),
            "fi:write".to_string(),
            "mm:read".to_string(),
        ];
        let token =
            generate_token(user_id, "multi", roles.clone(), perms.clone(), "sec", 60).unwrap();
        let claims = validate_token(&token, "sec").unwrap();
        assert_eq!(claims.roles, roles);
        assert_eq!(claims.permissions, perms);
    }

    #[test]
    fn password_strength_exactly_8_chars() {
        assert!(validate_password_strength("Abcdef1x").is_ok());
    }

    #[test]
    fn password_strength_special_chars_ok() {
        assert!(validate_password_strength("P@ssw0rd!").is_ok());
    }

    #[test]
    fn password_strength_unicode_no_digit() {
        assert!(validate_password_strength("Abcdefgh").is_err());
    }

    #[test]
    fn hash_password_different_salts() {
        let h1 = hash_password("same_pass").unwrap();
        let h2 = hash_password("same_pass").unwrap();
        assert_ne!(h1, h2); // Different salts produce different hashes
                            // But both should verify
        assert!(verify_password("same_pass", &h1).unwrap());
        assert!(verify_password("same_pass", &h2).unwrap());
    }
}
