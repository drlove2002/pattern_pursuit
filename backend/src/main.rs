mod authenticate_token;
mod config;
mod google_oauth;
mod handler;
mod model;

use actix_cors::Cors;
use actix_files::Files as fs;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use dotenv::dotenv;
use model::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    let db = AppState::init();
    let app_data = web::Data::new(db);

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&app_data.env.client_origin)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(app_data.clone())
            .configure(handler::config)
            .service(fs::new("/", "../frontend/static").index_file("index.html"))
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
