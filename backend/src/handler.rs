use crate::{
    middlewares::authenticate_token::AuthenticationGuard,
    model::{AppState, LbData, LbResponse, QueryCode, ResponseMsg, UserResponse},
    utils::{
        gen_jwt_token,
        google_oauth::{get_google_user, request_token},
    },
};
use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie, SameSite},
    get, post, web, HttpRequest, HttpResponse, Responder,
};
use redis::AsyncCommands;
use redis::RedisResult;
use reqwest::{header::LOCATION, Url};
use slog::{debug, error};

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "I'm up and running!";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}

// Get top 5 users from the leaderboard and the user's position
#[get("/leaderboard")]
async fn leaderboard_handler(
    auth_guard: AuthenticationGuard,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut conn = data.redis.get_conn_async();

    // Get top 5 users from the leaderboard
    let top_5: RedisResult<Vec<(String, u32)>> =
        conn.zrevrange_withscores("leaderboard", 0, 4).await;

    // Get the user's position
    let user_position: RedisResult<Option<u32>> = conn
        .zrevrank("leaderboard", auth_guard.user_id.to_owned())
        .await;
    let user_rating: RedisResult<Option<u32>> = conn
        .zscore("leaderboard", auth_guard.user_id.to_owned())
        .await;

    if top_5.is_err() || user_position.is_err() || user_rating.is_err() {
        error!(data.log, "Redis error"; "error" => top_5.err().unwrap().to_string());
        return HttpResponse::InternalServerError().json(ResponseMsg {
            status: "error".to_string(),
            message: "Internal server error".to_string(),
        });
    }

    let mut user_ids = top_5.unwrap();
    user_ids.push((auth_guard.user_id.to_owned(), user_rating.unwrap().unwrap()));
    let user_position = user_position.unwrap();
    let mut users: Vec<LbResponse> = Vec::new();

    let mut rank: u32 = 1;
    for (user_id, rating) in user_ids {
        if user_id == auth_guard.user_id {
            rank = user_position.unwrap() + 1;
        }
        let user_data: RedisResult<Vec<u32>> = conn
            .hget(
                format!("user:{}", user_id),
                &["accuracy", "highscore", "steps"],
            )
            .await;
        let profile = data
            .redis
            .get_profile(user_id.to_owned(), &data)
            .await
            .unwrap();
        match user_data {
            Ok(user_data) => {
                let user = LbResponse {
                    rank,
                    rating,
                    name: profile.name,
                    pfp: profile.picture,
                    accuracy: user_data[0],
                    highscore: user_data[1],
                    steps: user_data[2],
                };
                users.push(user);
                rank += 1;
            }
            Err(e) => {
                error!(data.log, "Redis error"; "error" => e.to_string());
                return HttpResponse::InternalServerError().json(ResponseMsg {
                    status: "error".to_string(),
                    message: "Internal server error".to_string(),
                });
            }
        }
    }

    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "Leaderboard fetched",
        "data": users}))
}

// Upload data to the leaderboard
#[post("/leaderboard")]
async fn upload_leaderboard_handler(
    auth_guard: AuthenticationGuard,
    data: web::Data<AppState>,
    body: web::Json<LbData>,
) -> impl Responder {
    let user_id = auth_guard.user_id.to_owned();
    let body = body.into_inner();
    let mut conn = data.redis.get_conn_async();

    debug!(
        data.log,
        "Leaderboard data";
        "user_id" => &user_id,
        "accuracy" => body.accuracy,
        "highscore" => body.highest_earning,
        "steps" => body.steps
    );
    // Upload name accurecy and highscore to the permanet user data
    let _: RedisResult<()> = conn
        .hset_multiple(
            format!("user:{}", user_id.to_owned()),
            &[
                ("accuracy", body.accuracy.to_string()),
                ("highscore", body.highest_earning.to_string()),
                ("steps", body.steps.to_string()),
            ],
        )
        .await;

    // Calculate the new leaderboard score
    let rating = if body.highest_earning < 1000 {
        100 - body.accuracy + body.steps
    } else if body.highest_earning == 2000 {
        body.highest_earning + (100 - body.accuracy) + body.steps
    } else {
        (body.highest_earning - 1000) + (100 - body.accuracy) + body.steps
    };

    // Get the previous score and compare it with the new score
    let prev_score: RedisResult<Option<u32>> = conn.zscore("leaderboard", user_id.to_owned()).await;
    if prev_score.is_err() {
        error!(data.log, "Redis error"; "error" => prev_score.err().unwrap().to_string());
        return HttpResponse::InternalServerError().json(ResponseMsg {
            status: "error".to_string(),
            message: "Internal server error".to_string(),
        });
    }
    let prev_score = prev_score.unwrap();
    if prev_score.is_some() && (prev_score.unwrap() > rating) {
        return HttpResponse::Ok().json(ResponseMsg {
            status: "success".to_string(),
            message: "Leaderboard updated".to_string(),
        });
    }

    // Upload the new score to the leaderboard
    let result: RedisResult<usize> = conn.zadd("leaderboard", user_id.to_owned(), rating).await;
    match result {
        Ok(_) => HttpResponse::Ok().json(ResponseMsg {
            status: "success".to_string(),
            message: "Leaderboard updated".to_string(),
        }),
        Err(e) => {
            error!(data.log, "Redis error"; "error" => e.to_string());
            HttpResponse::InternalServerError().json(ResponseMsg {
                status: "error".to_string(),
                message: "Internal server error".to_string(),
            })
        }
    }
}

#[get("/login")]
async fn oauth_url_handler(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    // Get login cookies
    let cookie = req.cookie("login");
    if cookie.is_some() && cookie.unwrap().value() == "true" {
        return HttpResponse::Found()
            .append_header((LOCATION, "/play.html"))
            .finish();
    }

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
        .same_site(SameSite::Strict)
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
        .same_site(SameSite::Strict)
        .finish();
    let cookie2 = Cookie::build("login", "true")
        .path("/")
        .max_age(ActixWebDuration::new(data.config.jwt_max_age, 0))
        .http_only(false)
        .finish();

    HttpResponse::Found()
        .append_header((LOCATION, "/play.html"))
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
        .same_site(SameSite::Strict)
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
        .get_profile(auth_guard.user_id.to_owned(), &data)
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
            .service(upload_leaderboard_handler)
            .service(leaderboard_handler)
            .service(get_me_handler),
    );
}
