use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn heathcheck() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/heathcheck", web::get().to(heathcheck)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
