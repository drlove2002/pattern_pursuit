pub mod config;
pub mod google_oauth;
pub mod logging;
pub mod redis;

use crate::model::{AppState, TokenClaims};
use actix_web::web;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

/// Generate a JWT token for the user
pub fn gen_jwt_token(user_id: String, data: &web::Data<AppState>) -> String {
    let jwt_secret = &data.config.jwt_secret;
    let now = Utc::now();
    let claims: TokenClaims = TokenClaims {
        sub: user_id,
        exp: (now + Duration::minutes(data.config.jwt_max_age)).timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap()
}
