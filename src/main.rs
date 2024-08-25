use actix_web::{get, post, web, App, HttpResponce, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponce::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
