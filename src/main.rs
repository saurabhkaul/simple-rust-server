use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::http::uri::Port;
mod utils;
use utils::constants::SERVER_ADDRESS;
mod controllers;
use controllers::routes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at {}",SERVER_ADDRESS);
    HttpServer::new(|| {
        App::new()
            .configure(routes)
    })
        .bind(SERVER_ADDRESS)?
        .run()
        .await
}