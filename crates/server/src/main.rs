#![allow(unused)]

mod client;
mod error;

use client::Client;

use actix_web::{
    get,
    middleware::Logger,
    web::{self},
    App, HttpServer, Result,
};
use env_logger::Env;
use sea_orm::{Database, DatabaseConnection};

use crate::{client::FetchResources, error::AppError};

#[get("/run")]
async fn run(client: web::Data<Client>) -> Result<String, AppError> {
    let resources = client.fetch_non_unique(30).await?;

    Ok(format!("{:?}", resources))
}

#[actix_web::main]
// async fn main() -> std::io::Result<()> {
async fn main() {
    // env_logger::init_from_env(Env::default().default_filter_or("info"));

    // HttpServer::new(|| {
    //     App::new()
    //         .service(run)
    //         .app_data(web::Data::new(client::Client::new("https://httpbin.org")))
    //         .wrap(Logger::default())
    //         .wrap(Logger::new("%a %{User-Agent}i"))
    // })
    // .bind(("127.0.0.1", 8000))?
    // .run()
    // .await

    // let db: DatabaseConnection = Database::connect("mysql://root:my-secret-pw@localhost:3306")
    //     .await
    //     .unwrap();

    // println!("{:?}", db);
}
