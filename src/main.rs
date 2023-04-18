
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


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");
HttpServer::new(|| { 
    App::new().service(
    // prefixes all resources and routes attached to it...
    web::scope("/api")
        // ...so this handles requests for `Post /api/session`
        .route("/session", web::post().to(session))
        )
        // static file routing
        .route("/", web::get().to(index))
        .route("/ws", web::get().to(ws_handler))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().route("/ws/", web::get().to(index)))
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }