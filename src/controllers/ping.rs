use actix_web::HttpResponse;

pub async fn ping()->HttpResponse{
    HttpResponse::Ok().set_header("hidden-message","fuck off").body("Pong. Now See hidden message")
}

