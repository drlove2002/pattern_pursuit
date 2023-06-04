use crate::{
    middlewares::authenticate_token::AuthenticationGuard,
    model::{AppState, QueryCode, ResponseMsg, UserResponse},
    utils::{
        gen_jwt_token,
        google_oauth::{get_google_user, request_token},
    },
};
use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, web, HttpResponse, Responder,
};
use redis::AsyncCommands;
use redis::RedisResult;
use reqwest::{header::LOCATION, Url};
use slog::debug;

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "I'm up and running!";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}

#[get("/login")]
async fn oauth_url_handler(data: web::Data<AppState>) -> impl Responder {
    let mut url = Url::parse("https://accounts.google.com/o/oauth2/v2/auth").unwrap();

    url.query_pairs_mut()
        .append_pair("client_id", &data.config.google_oauth_client_id)
        .append_pair("redirect_uri", &data.config.google_oauth_redirect_url)
        .append_pair("response_type", "code")
        .append_pair("scope", "email profile")
        .append_pair("state", "random_string")
        .append_pair("access_type", "offline")
        .append_pair("prompt", "consent");

    HttpResponse::Found()
        .append_header((LOCATION, url.as_str()))
        .finish()
}

#[get("/refresh")]
async fn jwt_refresh_handler(
    auth_guard: AuthenticationGuard,
    data: web::Data<AppState>,
) -> impl Responder {
    let token: String = gen_jwt_token(auth_guard.user_id.to_owned(), &data);
    data.redis
        .update_profile_ttl(auth_guard.user_id.to_owned())
        .await;
    debug!(data.log, "Token refreshed"; "user_id" => auth_guard.user_id);

    let cookie = Cookie::build("token", token)
        .path("/")
        .max_age(ActixWebDuration::new(data.config.jwt_max_age, 0))
        .http_only(true)
        .finish();
    let cookie2 = Cookie::build("login", "true")
        .path("/")
        .max_age(ActixWebDuration::new(data.config.jwt_max_age, 0))
        .http_only(false)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .cookie(cookie2)
        .json(ResponseMsg {
            status: "success".to_string(),
            message: "Token refreshed".to_string(),
        })
}

#[get("/oauth/login")]
async fn google_oauth_handler(
    query: web::Query<QueryCode>,
    data: web::Data<AppState>,
) -> impl Responder {
    let code = &query.code;

    if code.is_empty() {
        return HttpResponse::Unauthorized().json(
            serde_json::json!({"status": "fail", "message": "Authorization code not provided!"}),
        );
    }

    let token_response = request_token(code.as_str(), &data).await;
    if token_response.is_err() {
        return HttpResponse::BadGateway().json(ResponseMsg {
            status: "fail".to_string(),
            message: token_response.err().unwrap().to_string(),
        });
    }

    let token_response = token_response.unwrap();
    let user = get_google_user(
        &token_response.access_token,
        &token_response.id_token,
        &data,
    )
    .await;
    if user.is_err() {
        return HttpResponse::BadGateway().json(ResponseMsg {
            status: "fail".to_string(),
            message: user.err().unwrap().to_string(),
        });
    }

    let user = user.unwrap();

    {
        let mut con = data.redis.get_conn_async();

        let result: RedisResult<u8> = con
            .hset(
                format!("user:{}", user.id),
                "refresh_token",
                &token_response.refresh_token,
            )
            .await;
        match result {
            Ok(_) => {
                debug!(data.log, "Refresh token saved to redis"; "user_id" => user.id.to_owned());
            }
            Err(e) => {
                debug!(data.log, "Failed to save refresh token to redis"; "user_id" => user.id.to_owned(), "error" => e.to_string());
            }
        }
    }
    data.redis.set_profile(user.to_owned()).await;
    let token: String = gen_jwt_token(user.id, &data);
    let cookie = Cookie::build("token", token)
        .path("/")
        .max_age(ActixWebDuration::new(data.config.jwt_max_age, 0))
        .http_only(true)
        .finish();
    let cookie2 = Cookie::build("login", "true")
        .path("/")
        .max_age(ActixWebDuration::new(data.config.jwt_max_age, 0))
        .http_only(false)
        .finish();

    HttpResponse::Found()
        .append_header((LOCATION, data.config.client_origin.to_owned()))
        .cookie(cookie)
        .cookie(cookie2)
        .finish()
}

#[get("/oauth/logout")]
async fn logout_handler(
    auth_guard: AuthenticationGuard,
    data: web::Data<AppState>,
) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    let cookie2 = Cookie::build("login", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(false)
        .finish();

    // Remove user from db
    data.redis
        .delete_profile(auth_guard.user_id.to_owned())
        .await;
    let mut con = data.redis.get_conn_async();
    let _: RedisResult<u8> = con
        .hdel(format!("user:{}", auth_guard.user_id), "refresh_token")
        .await;

    HttpResponse::Found()
        .cookie(cookie)
        .cookie(cookie2)
        .append_header((LOCATION, "/"))
        .finish()
}

#[get("/users/me")]
async fn get_me_handler(
    auth_guard: AuthenticationGuard,
    data: web::Data<AppState>,
) -> impl Responder {
    let user = data
        .redis
        .get_profile(auth_guard.user_id.to_owned())
        .await
        .unwrap();
    let json_response = UserResponse {
        status: "success".to_string(),
        data: user,
    };
    HttpResponse::Ok().json(json_response)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(health_checker_handler)
            .service(jwt_refresh_handler)
            .service(oauth_url_handler)
            .service(google_oauth_handler)
            .service(logout_handler)
            .service(get_me_handler),
    );
}
