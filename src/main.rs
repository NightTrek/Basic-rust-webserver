
use actix_web::{web, App, HttpServer};
use actix_files::NamedFile;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

mod real_time_handler;
use real_time_handler::{ws_handler, SessionsStorage, Session, Position};



async fn index() -> std::io::Result<NamedFile> {
    println!("[200] GET / index.html");
    let path: PathBuf = "./static/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");
    let position = Position { x: 1, y: 2 };
    let session = Session {
        positions: vec![position],
    };
    let sessions_storage = Arc::new(Mutex::new(SessionsStorage {
        sessions: Mutex::new(vec![session]),
    }));

     // Note: web::Data created _outside_ HttpServer::new closure


HttpServer::new(move || { 
    App::new()
        .app_data(Arc::clone(&sessions_storage)) // <- register the created data
        .route("/ws", web::get().to(ws_handler)) // websocket handler with access to state

        // static file routing
        .route("/", web::get().to(index))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}