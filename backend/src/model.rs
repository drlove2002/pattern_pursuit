use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::utils::{config::AppConfig, logging};

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
    pub conf: AppConfig,
    pub log: slog::Logger,
    pub http: reqwest::Client,
}

impl AppState {
    pub fn init() -> AppState {
        let state = AppState {
            db: Arc::new(Mutex::new(Vec::new())),
            conf: AppConfig::init(),
            log: logging::config(),
            http: reqwest::Client::new(),
        };
        slog::info!(state.log, "✅ App state initialized successfully");
        state
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
