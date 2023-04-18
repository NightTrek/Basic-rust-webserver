
use actix_web::{web, App,  HttpResponse, HttpServer};
use actix_files::NamedFile;
use std::path::PathBuf;
mod real_time_handler;
use real_time_handler::ws_handler;

async fn index() -> std::io::Result<NamedFile> {
    println!("[200] GET / index.html");
    let path: PathBuf = "./static/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}


async fn session() -> HttpResponse {
    println!("[200] POST /api/session");
    HttpResponse::Ok().body("Session endpoint")
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
        .service(web::resource("/ws/").to(ws_handler))
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