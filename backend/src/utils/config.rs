#[derive(Debug, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub client_origin: String,
    pub jwt_secret: String,
    pub reids_cache_ttl: usize,
    pub jwt_max_age: i64,
    pub google_oauth_client_id: String,
    pub google_oauth_client_secret: String,
    pub google_oauth_redirect_url: String,
}

impl AppConfig {
    pub fn init(log: &slog::Logger) -> AppConfig {
        let port = match std::env::var("PORT") {
            Ok(port) => port.parse::<u16>().unwrap(),
            Err(_) => 3000,
        };
        let client_origin = std::env::var("CLIENT_ORIGIN").expect("CLIENT_ORIGIN must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let reids_cache_ttl: usize = 60 * 60; // 1 hour
        let jwt_max_age: i64 = 60 * 60 * 24 * 30 * 4; // 4 months
        let google_oauth_client_id =
            std::env::var("GOOGLE_OAUTH_CLIENT_ID").expect("GOOGLE_OAUTH_CLIENT_ID must be set");
        let google_oauth_client_secret = std::env::var("GOOGLE_OAUTH_CLIENT_SECRET")
            .expect("GOOGLE_OAUTH_CLIENT_SECRET must be set");
        let google_oauth_redirect_url = std::env::var("GOOGLE_OAUTH_REDIRECT_URL")
            .expect("GOOGLE_OAUTH_REDIRECT_URL must be set");

        slog::info!(log, "✅ App config initialized successfully");
        AppConfig {
            port,
            client_origin,
            jwt_secret,
            reids_cache_ttl,
            jwt_max_age,
            google_oauth_client_id,
            google_oauth_client_secret,
            google_oauth_redirect_url,
        }
    }
}
