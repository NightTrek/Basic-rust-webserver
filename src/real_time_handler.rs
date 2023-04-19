
use actix::prelude::*;
use actix::{Actor, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::sync::{Arc,Mutex, MutexGuard};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Deserialize)]
pub struct ClientMessage {
    pub x: i32,
    pub y: i32,
    pub session_id: usize,
}

impl Default for ClientMessage {
    fn default() -> ClientMessage {
        ClientMessage {
            x: 0,
            y: 0,
            session_id: 0,
        }
    }
}
#[derive(Serialize, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
#[derive(Serialize, Debug)]
pub struct Session {
    pub positions: Vec<Position>,
}
#[derive(Serialize, Debug)]
pub struct SessionsStorage {
    pub sessions: Mutex<Vec<Session>>, // <- Mutex is necessary to mutate safely across threads
}

/// Define HTTP actor
impl Actor for SessionsStorage {
    type Context = Context<Self>;
}

struct MyWs {
    sessions_storage: web::Data<Arc<Mutex<SessionsStorage>>>,
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            //print out the message and continue listening
            Ok(ws::Message::Text(text)) => {
                println!("Server got message: {}", text);
                let inputString = String::from(text);
                // convert the JSON string into a clientMessage struct
                let client_message: ClientMessage = serde_json::from_str(&inputString).unwrap_or_default();
                let allSessions = self.sessions_storage.lock().unwrap();
                let mut sessions:MutexGuard<Vec<Session>> = allSessions.sessions.lock().unwrap();
                sessions[client_message.session_id].positions.push(Position { x: client_message.x, y: client_message.y });
                
                for session in sessions.iter() {
                    println!("session: {:?}", session);
                }
                // let json_bytes = serde_json::to_vec(&session_copy).unwrap();

                ctx.text("{\"msg\": \"completed update\"}")            } 

            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

pub async fn ws_handler(req: HttpRequest, stream: web::Payload, data: web::Data<Arc<Mutex<SessionsStorage>>>) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {sessions_storage: data}, &req, stream);
    println!("{:?}", resp);
    resp
}