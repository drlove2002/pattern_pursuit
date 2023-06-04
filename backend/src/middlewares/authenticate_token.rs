use actix_web::{
    dev::Payload,
    error::{Error as ActixWebError, ErrorUnauthorized},
    http, web, FromRequest, HttpRequest,
};
use futures::Future;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use redis::{AsyncCommands, RedisResult};
use serde_json::json;
use slog::debug;
use std::pin::Pin;

use crate::{
    model::{AppState, TokenClaims},
    utils::google_oauth::{get_google_user, request_access_token},
};

pub struct AuthenticationGuard {
    pub user_id: String,
}

impl FromRequest for AuthenticationGuard {
    type Error = ActixWebError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        if token.is_none() {
            return Box::pin(async move {
                Err(ErrorUnauthorized(
                    json!({"status": "fail", "message": "You are not logged in, please provide token"}),
                ))
            });
        }

        let req = req.clone();
        let data = req.app_data::<web::Data<AppState>>().unwrap().clone();

        let jwt_secret = data.config.jwt_secret.to_owned();
        let decode = decode::<TokenClaims>(
            token.unwrap().as_str(),
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        );

        let mut conn = data.redis.get_conn_async();
        Box::pin(async move {
            match decode {
                Ok(token) => {
                    let key = format!("profile:{}", token.claims.sub);
                    let user_exists: bool = conn.exists(key).await.unwrap();
                    debug!(data.log, "User found?: {:?}", user_exists);
                    if !user_exists {
                        let refresh_token: RedisResult<String> = conn
                            .hget(format!("user:{}", token.claims.sub), "refresh_token")
                            .await;
                        let access_token = match refresh_token {
                            Ok(refresh_token) => {
                                request_access_token(refresh_token.as_str(), &data).await
                            }
                            Err(_) => {
                                return Err(ErrorUnauthorized(
                                    json!({"status": "fail", "message": "Failed to retrieve refresh token"}),
                                ))
                            }
                        };
                        let access_token = match access_token {
                            Ok(access_token) => access_token,
                            Err(_) => {
                                return Err(ErrorUnauthorized(
                                    json!({"status": "fail", "message": "Failed to retrieve access token"}),
                                ))
                            }
                        };

                        let user = match get_google_user(&access_token, &access_token, &data).await
                        {
                            Ok(user) => user,
                            Err(_) => {
                                return Err(ErrorUnauthorized(
                                    json!({"status": "fail", "message": "Failed to retrieve user data"}),
                                ))
                            }
                        };

                        data.redis.set_profile(user.to_owned()).await;
                    }
                    Ok(AuthenticationGuard {
                        user_id: token.claims.sub,
                    })
                }
                Err(_) => Err(ErrorUnauthorized(
                    json!({"status": "fail", "message": "Invalid token or user doesn't exists"}),
                )),
            }
        })
    }

    fn extract(req: &HttpRequest) -> Self::Future {
        Self::from_request(req, &mut Payload::None)
    }
}
