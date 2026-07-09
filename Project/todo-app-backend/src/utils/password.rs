use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

/// Hash a plain text password
pub fn hash_password(password: &str) -> Result<String, password_hash::Error> {
    // Generate a random salt
    let salt = SaltString::generate(&mut OsRng);

    // Default Argon2 configuration
    let argon2 = Argon2::default();

    // Hash password
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

/// Verify a password against the stored hash
pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, password_hash::Error> {
    let parsed_hash = PasswordHash::new(password_hash)?;

    let argon2 = Argon2::default();

    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
