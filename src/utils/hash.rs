use bcrypt::{DEFAULT_COST, hash, verify};

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(hashed: &str, password: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hashed)
}
