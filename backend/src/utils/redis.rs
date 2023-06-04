use crate::model::Profile;
use redis::{aio::MultiplexedConnection, AsyncCommands, Client};
use slog::{debug, error, info, Logger};

pub struct RedisClient {
    con: MultiplexedConnection,
    log: Logger,
    max_ttl: usize,
}

impl RedisClient {
    pub async fn init(log: Logger, ttl: usize) -> RedisClient {
        let client = Client::open(std::env::var("REDIS_URL").unwrap()).unwrap();
        let con = client.get_multiplexed_tokio_connection().await.unwrap();
        info!(log, "✅ Redis client initialized successfully");
        RedisClient {
            con,
            log,
            max_ttl: ttl,
        }
    }

    /// Get async connection from Redis
    pub fn get_conn_async(&self) -> MultiplexedConnection {
        self.con.clone()
    }

    /// Get profile(id, name, email, photo) from Redis
    pub async fn get_profile(&self, id: String) -> Option<Profile> {
        let mut con = self.con.clone();
        let key = format!("profile:{}", id);
        let profile: Profile = match con.get(key).await {
            Ok(res) => match res {
                Some(val) => val,
                None => return None,
            },
            Err(err) => {
                error!(self.log, "❌ Redis error: {}", err);
                return None;
            }
        };
        Some(profile)
    }

    /// Set profile(id, name, email, photo) in Redis with ttl
    pub async fn set_profile(&self, user: Profile) {
        let mut con = self.con.clone();
        // Set profile in Redis with ttl
        match con
            .set_ex(format!("profile:{}", user.id), user, self.max_ttl)
            .await
        {
            Ok(res) => res,
            Err(err) => error!(self.log, "❌ Redis error: {}", err),
        };

        debug!(self.log, "✅ Redis set profile");
    }

    /// Update ttl of profile in Redis
    pub async fn update_profile_ttl(&self, user_id: String) {
        let mut con = self.con.clone();
        match con
            .expire(format!("profile:{}", user_id), self.max_ttl)
            .await
        {
            Ok(res) => res,
            Err(err) => error!(self.log, "❌ Redis error: {}", err),
        };
    }

    /// Delete profile(id, name, email, photo) from Redis
    pub async fn delete_profile(&self, id: String) {
        let mut con = self.con.clone();
        match con.del(format!("profile:{}", id)).await {
            Ok(res) => res,
            Err(err) => error!(self.log, "❌ Redis error: {}", err),
        };
    }
}
