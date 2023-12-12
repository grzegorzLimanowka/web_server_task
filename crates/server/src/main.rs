mod client;
mod db;
mod error;
mod routes;

use std::env;

use client::{Client, Fetcher};

use actix_web::{
    middleware::Logger,
    web::{self},
    App, HttpServer,
};
use env_logger::Env;
use routes::run::run;
use sea_orm::Database;

#[derive(Debug, Clone)]
struct AppState {
    // conn: DatabaseConnection,
    // client: Client,
    fetcher: Fetcher<Client>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not present as ENVVAR");
    let host = env::var("HOST").expect("HOST is not set as ENVVAR");
    let port = env::var("PORT").expect("PORT is not set as ENVVAR");
    let server_url = format!("{host}:{port}");

    let conn = Database::connect(db_url)
        .await
        .expect("Couldnt connect to db !");

    let client = client::Client::new("https://httpbin.org");
    let fetcher = Fetcher { client, conn };

    let state = AppState { fetcher };

    HttpServer::new(move || {
        App::new()
            .service(run)
            .app_data(web::Data::new(state.clone()))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(server_url)?
    .run()
    .await
}
