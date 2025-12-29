mod api;
mod data;
mod errors;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer, middleware};
use log::info;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let api_state = Arc::new(api::ApiClient::new());

    info!("Starting World Bank API server on http://0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(api_state.clone()))
            .wrap(middleware::Logger::default())
            .wrap(actix_web::middleware::NormalizePath::trim())
            .service(
                web::scope("/api")
                    .route("/countries", web::get().to(handlers::get_countries))
                    .route("/indicators", web::get().to(handlers::get_indicators))
                    .route("/data/{country_id}/{indicator_id}", web::get().to(handlers::get_data))
                    .route("/compare", web::post().to(handlers::compare_countries))
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
