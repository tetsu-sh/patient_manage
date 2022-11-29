pub use bcrypt::verify;
use bcrypt::{hash, DEFAULT_COST};

pub fn hash_password(raw_password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(raw_password, DEFAULT_COST)
}
