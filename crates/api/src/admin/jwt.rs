use jsonwebtoken::{decode, encode, DecodingKey, Header, Validation};

use crate::errors::ApiError;

struct Claims {
    sub: String,
    exp: u64,
}

pub fn create_token(sub: String) -> Result<String, ApiError> {
    let secret = std::env::var("JWT_SECRET").map_err(|_| ApiError::Internal)?;

    let exp = secs_from_now(3600);
    let claims = Claims { sub, exp };

    let token = encode(&Header::default(), &claims, &secret).map_err(|_| ApiError::Internal)?;

    Ok(token)
}

pub fn validate_token(token: String) -> Result<(), ApiError> {
    let secret = std::env::var("JWT_SECRET").map_err(|_| ApiError::Internal)?;
    let key = DecodingKey::from_secret(secret);

    decode(&token, &key, &Validation::default()).map_err(|_| ApiError::InvalidToken)?;

    Ok(())
}

fn secs_from_now(seconds: u64) -> u64 {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");

    (now + Duration::from_secs(seconds)).as_secs()
}
