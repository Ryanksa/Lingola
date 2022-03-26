use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[post("/")]
async fn index(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 5000))?
        .run()
        .await
}
