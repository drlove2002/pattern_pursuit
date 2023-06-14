mod handler;
mod middlewares;
mod model;
mod utils;

use actix_cors::Cors;
use actix_files::Files as fs;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use dotenv::dotenv;
use model::AppState;
use slog::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    // dotenv is only needed for local development
    dotenv().ok();
    env_logger::init();

    let app_data = web::Data::new(AppState::init().await);

    let port = app_data.config.port;

    info!(app_data.log, "🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
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
            .wrap(Logger::new("%r %s %b %{Referer}i %T").log_target("actix_web"))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
