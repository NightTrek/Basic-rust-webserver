
use actix_web::{web, App,  HttpResponse, HttpServer};
// mod realTimeHandler;


fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

fn session() -> HttpResponse {
    HttpResponse::Ok().body("Session endpoint")
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::resource("/session").route(web::post().to(session))),
    ).service(
        web::resource("/")
            .route(web::get().to(index))
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");
HttpServer::new(|| App::new().configure(config))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}