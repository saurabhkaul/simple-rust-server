use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod ping;
use ping::ping;


pub fn routes (app :&mut web::ServiceConfig){
    app.service(web::resource("/ping").to(ping));
}