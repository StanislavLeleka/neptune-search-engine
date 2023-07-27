use actix_web::{middleware::Logger, web::Data, App, HttpServer};

use env_logger::Env;
use repo::mongodb_repo::MongoRepo;

pub mod app;
pub mod config;
pub mod models;
pub mod repo;
pub mod routes;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let repo = MongoRepo::new().await.unwrap();
    let db_data = Data::new(repo);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(db_data.clone())
            .configure(routes::api)
    })
    .bind(("0.0.0.0", 8084))?
    .run()
    .await
}
