
use actix::prelude::*;
use actix::{Actor, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::sync::Mutex;


pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct Session {
    pub positions: Vec<Position>,
}
pub struct SessionsStorage {
    pub sessions: Mutex<Vec<Session>>, // <- Mutex is necessary to mutate safely across threads
}

/// Define HTTP actor
impl Actor for SessionsStorage {
    type Context = Context<Self>;
}

struct MyWs {
    sessions_storage: Mutex<Vec<real_time_handler::Session>>,
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

impl MyWs {
    fn new(sessions_storage: Mutex<Vec<real_time_handler::Session>>) -> Self {
        Self { sessions_storage }
    }
}
/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            //print out the message and continue listening
            Ok(ws::Message::Text(text)) => {
                println!("Server got message: {}", text);
                ctx.text(text)
            } 

            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

pub async fn ws_handler(data: web::Data<SessionsStorage>, req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let mut server_sessions = data.sessions.lock().unwrap(); // <- get counter's MutexGuard
    // get the length of the sessions
    let len = server_sessions.len();
    if len > 100 {
        // remove the oldest session
        println!("Removing oldest session");
        server_sessions.remove(0);
    }
    let resp = ws::start(MyWs {sessions_storage: data.sessions}, &req, stream);
    println!("{:?}", resp);
    resp
}