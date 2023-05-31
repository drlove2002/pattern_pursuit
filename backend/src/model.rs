use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::config;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub photo: String,
}

pub struct AppState {
    pub db: Arc<Mutex<Vec<User>>>,
    pub env: config::Config,
}

impl AppState {
    pub fn init() -> AppState {
        AppState {
            db: Arc::new(Mutex::new(Vec::new())),
            env: config::Config::init(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct QueryCode {
    pub code: String,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: User,
}
