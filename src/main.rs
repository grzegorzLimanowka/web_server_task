#![allow(unused)]

mod client;
mod error;

use client::Client;
use client::NonUnique;

use std::sync::Mutex;

use url::{ParseError, Url};

use actix_web::{
    get, guard,
    middleware::Logger,
    post,
    web::{self, ServiceConfig},
    App, HttpResponse, HttpServer, Responder, Result,
};
use env_logger::Env;

use crate::error::AppError;

// TODO: Share client?

#[get("/run")]
async fn run() -> Result<String, AppError> {
    // let number = 0;

    let url = Url::parse("https://httpbin.org/post")?;

    let client = client::Client::new();

    let number = client.non_unique(url, 2).await.unwrap();

    Ok(format!("{:?}", number))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .service(run)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
