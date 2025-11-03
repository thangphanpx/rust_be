use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Claims {
    pub sub: String, // Subject (user identifier)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
}

#[allow(dead_code)]
impl Claims {
    pub fn new(user_id: i32) -> Self {
        let now = Utc::now();
        let expiration = now + Duration::hours(24); // Token expires in 24 hours

        Self {
            sub: user_id.to_string(),
            exp: expiration.timestamp() as usize,
            iat: now.timestamp() as usize,
        }
    }
}

#[allow(dead_code)]
pub fn create_jwt(user_id: i32, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(user_id);
    let key = EncodingKey::from_secret(secret.as_ref());
    encode(&Header::default(), &claims, &key)
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let key = DecodingKey::from_secret(secret.as_ref());
    let validation = Validation::new(Algorithm::HS256);
    
    decode::<Claims>(token, &key, &validation)
        .map(|data| data.claims)
}