use crate::utils::{config::AppConfig, logging, redis::RedisClient};
use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, FromRedisValue, ToRedisArgs)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub email: String,
    pub picture: String,
}

pub struct AppState {
    pub redis: RedisClient,
    pub config: AppConfig,
    pub log: slog::Logger,
    pub http: reqwest::Client,
}

impl AppState {
    pub async fn init() -> AppState {
        let log = logging::config();
        let config = AppConfig::init(&log);
        let redis = RedisClient::init(log.to_owned(), config.reids_cache_ttl).await;
        let state = AppState {
            redis,
            config,
            log: log.to_owned(),
            http: reqwest::Client::new(),
        };
        slog::info!(log, "✅ App state initialized successfully");
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
pub struct LbData {
    pub accuracy: u32,
    pub highest_earning: u32,
    pub steps: u32,
}

#[derive(Debug, Deserialize)]
pub struct QueryCode {
    pub code: String,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: Profile,
}

#[derive(Serialize, Debug)]
pub struct LbResponse {
    pub rank: u8,
    pub rating: u32,
    pub name: String,
    pub pfp: String,
    pub accuracy: u32,
    pub highscore: u32,
    pub steps: u32,
}

#[derive(Serialize, Debug)]
pub struct ResponseMsg {
    pub status: String,
    pub message: String,
}
