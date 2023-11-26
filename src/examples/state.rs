#![allow(unused)]

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // get app name
    format!("Hello {app_name}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        // App::new().service(web::scope("/app").route("/index.html", web::get().to(index)))
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: "Actix web".to_owned(),
            }))
            .service(index)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
