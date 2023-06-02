use crate::{
    middlewares::authenticate_token::AuthenticationGuard,
    model::{AppState, QueryCode, User, UserResponse},
    utils::{
        gen_jwt_token,
        google_oauth::{get_google_user, request_token},
    },
};
use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, web, HttpResponse, Responder,
};
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
        .append_pair("client_id", &data.conf.google_oauth_client_id)
        .append_pair("redirect_uri", &data.conf.google_oauth_redirect_url)
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
async fn token_refresh_handler(
    auth_guard: AuthenticationGuard,
    data: web::Data<AppState>,
) -> impl Responder {
    let token: String = gen_jwt_token(auth_guard.user_id.to_owned(), &data);
    debug!(data.log, "Token refreshed"; "user_id" => auth_guard.user_id);

    let cookie = Cookie::build("token", token)
        .path("/")
        .max_age(ActixWebDuration::new(60 * data.conf.jwt_max_age, 0))
        .http_only(true)
        .finish();
    let cookie2 = Cookie::build("login", "true")
        .path("/")
        .max_age(ActixWebDuration::new(60 * data.conf.jwt_max_age, 0))
        .http_only(false)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .cookie(cookie2)
        .json(serde_json::json!({"status": "success", "message": "Token refreshed"}))
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
        let message = token_response.err().unwrap().to_string();
        return HttpResponse::BadGateway()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let token_response = token_response.unwrap();
    let google_user = get_google_user(
        &token_response.access_token,
        &token_response.id_token,
        &data,
    )
    .await;
    if google_user.is_err() {
        let message = google_user.err().unwrap().to_string();
        return HttpResponse::BadGateway()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let google_user = google_user.unwrap();

    let mut vec = data.db.lock().unwrap();
    let user_id = google_user.id.to_owned();
    let user = vec.iter_mut().find(|user| user.id == user_id);

    if user.is_some() {
        let user = user.unwrap();
        user.name = google_user.name;
        user.email = google_user.email.to_lowercase();
        user.photo = google_user.picture;
    } else {
        let user_data = User {
            id: google_user.id.to_owned(),
            name: google_user.name,
            email: google_user.email.to_lowercase(),
            photo: google_user.picture,
        };

        vec.push(user_data);
    }

    let token: String = gen_jwt_token(google_user.id, &data);
    let cookie = Cookie::build("token", token)
        .path("/")
        .max_age(ActixWebDuration::new(60 * data.conf.jwt_max_age, 0))
        .http_only(true)
        .finish();
    let cookie2 = Cookie::build("login", "true")
        .path("/")
        .max_age(ActixWebDuration::new(60 * data.conf.jwt_max_age, 0))
        .http_only(false)
        .finish();

    let frontend_origin = data.conf.client_origin.to_owned();
    let mut response = HttpResponse::Found();
    response
        .append_header((LOCATION, frontend_origin))
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
    let mut vec = data.db.lock().unwrap();
    if let Some(index) = vec.iter().position(|user| user.id == auth_guard.user_id) {
        vec.remove(index);
    }

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
    let vec = data.db.lock().unwrap();

    let user = vec.iter().find(|user| user.id == auth_guard.user_id);

    match user {
        Some(user) => {
            let json_response = UserResponse {
                status: "success".to_string(),
                data: user.to_owned(),
            };

            HttpResponse::Ok().json(json_response)
        }
        None => HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail", "message": "User not found!"})),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(health_checker_handler)
            .service(token_refresh_handler)
            .service(oauth_url_handler)
            .service(google_oauth_handler)
            .service(logout_handler)
            .service(get_me_handler),
    );
}
