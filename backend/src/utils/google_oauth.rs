use actix_web::web;
use reqwest::Url;
use serde::Deserialize;
use std::error::Error;

use crate::model::{AppState, Profile};

#[derive(Deserialize)]
pub struct OAuthResponse {
    pub access_token: String,
    pub id_token: String,
    pub refresh_token: String,
}

#[derive(Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: usize,
}

/// Get access token from Google using refresh token
pub async fn request_access_token(
    refresh_token: &str,
    data: &web::Data<AppState>,
) -> Result<String, Box<dyn Error>> {
    let root_url = "https://oauth2.googleapis.com/token";
    let client_secret = data.config.google_oauth_client_secret.to_owned();
    let client_id = data.config.google_oauth_client_id.to_owned();

    let params = [
        ("grant_type", "refresh_token"),
        ("client_id", client_id.as_str()),
        ("client_secret", client_secret.as_str()),
        ("refresh_token", refresh_token),
    ];
    let response = data.http.post(root_url).form(&params).send().await?;

    if response.status().is_success() {
        // Get access token from response and return it as a String
        let oauth_response = response.json::<TokenResponse>().await?;
        Ok(oauth_response.access_token)
    } else {
        let message = "An error occurred while trying to retrieve access token.";
        Err(From::from(message))
    }
}

pub async fn request_token(
    authorization_code: &str,
    data: &web::Data<AppState>,
) -> Result<OAuthResponse, Box<dyn Error>> {
    let redirect_url = data.config.google_oauth_redirect_url.to_owned();
    let client_secret = data.config.google_oauth_client_secret.to_owned();
    let client_id = data.config.google_oauth_client_id.to_owned();

    let root_url = "https://oauth2.googleapis.com/token";

    let params = [
        ("grant_type", "authorization_code"),
        ("redirect_uri", redirect_url.as_str()),
        ("client_id", client_id.as_str()),
        ("code", authorization_code),
        ("client_secret", client_secret.as_str()),
    ];
    let response = data.http.post(root_url).form(&params).send().await?;

    if response.status().is_success() {
        let oauth_response = response.json::<OAuthResponse>().await?;
        Ok(oauth_response)
    } else {
        let message = "An error occurred while trying to retrieve access token.";
        Err(From::from(message))
    }
}

pub async fn get_google_user(
    access_token: &str,
    id_token: &str,
    data: &web::Data<AppState>,
) -> Result<Profile, Box<dyn Error>> {
    let mut url = Url::parse("https://www.googleapis.com/oauth2/v1/userinfo").unwrap();
    url.query_pairs_mut().append_pair("alt", "json");
    url.query_pairs_mut()
        .append_pair("access_token", access_token);

    let response = data.http.get(url).bearer_auth(id_token).send().await?;

    if response.status().is_success() {
        // get google user data in raw json format
        let user_info = response.json::<Profile>().await?;
        Ok(user_info)
    } else {
        let message = "An error occurred while trying to retrieve user information.";
        Err(From::from(message))
    }
}
