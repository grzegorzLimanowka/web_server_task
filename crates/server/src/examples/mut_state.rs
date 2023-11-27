#![allow(unused)]

use std::sync::Mutex;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("request number: {counter}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

// Key takeaways:
//
// State initialized inside the closure passed to HttpServer::new is local to the worker thread and may become de-synced if modified.
// To achieve globally shared state, it must be created outside of the closure passed to HttpServer::new and moved/cloned in.
