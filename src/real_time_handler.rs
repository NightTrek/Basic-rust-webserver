
use actix::prelude::*;
use actix::{Actor, StreamHandler};
use actix_web::{
    web:: {Data, Payload},
     Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::{
    sync::{Arc,Mutex}
};
use bytestring::ByteString;
use serde::{Serialize, Deserialize};
use serde_json;


const MAX_HISTORY: usize = 10;
const MAX_CONNECTIONS: usize = 100;

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
pub struct ClientSessionStartMessage {
    pub session_id: usize,
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
pub struct SessionsStorage { // <- Mutex  around this is necessary to mutate safely across threads
    pub sessions: Vec<Session>, 
}

impl Default for SessionsStorage {
    fn default() -> SessionsStorage {
        SessionsStorage {
            sessions: vec![Session { positions: vec![Position { x: 0, y: 0 }] }],
        }
    }
}


/// Define HTTP actor
impl Actor for SessionsStorage {
    type Context = Context<Self>;
}

struct MyWs {
    session_id: usize,
    sessions_storage: Data<Arc<Mutex<SessionsStorage>>>,
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

// function which takes a ws::Message::text and mutable Sef::Context and returns a ws::Message::text
fn handle_position_update(text: ByteString, session_id: usize, session_storage: Data<Arc<Mutex<SessionsStorage>>>) -> String {
    let input_string = String::from(text);
    // convert the JSON string into a clientMessage struct
    let client_message: ClientMessage = serde_json::from_str(&input_string).unwrap_or_default();
    let mut all_sessions = session_storage.lock().unwrap(); // need beetter error handling here TODO ERROR HANDLING URGENT

    // check the length of the positions Vector and if its less than MAX_HISTORY, push the new position
    if all_sessions.sessions[session_id].positions.len() < MAX_HISTORY {
        all_sessions.sessions[session_id].positions.push(Position { x: client_message.x, y: client_message.y });
    } else {
        // if the length is 10, remove the first element and push the new position
        all_sessions.sessions[session_id].positions.remove(0);
        all_sessions.sessions[session_id].positions.push(Position { x: client_message.x, y: client_message.y });
    }
    // update the session storage
    // all_sessions.sessions[client_message.session_id].positions.push(Position { x: client_message.x, y: client_message.y });
    let sessions_read_only: &SessionsStorage = &*all_sessions;
    
    let json_bytes = serde_json::to_string(sessions_read_only).unwrap_or_default();
    return json_bytes;
}

fn handle_session_start( session_id: usize) -> String {
    let client_session_start_message = ClientSessionStartMessage { session_id: session_id };
    let json_string = serde_json::to_string(&client_session_start_message).unwrap_or_default();
    return json_string;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            //handle updating the position Array
            Ok(ws::Message::Text(text)) => {
                println!("Server got message: {}", text);
                println!("session_id: {}", self.session_id);
                if text == "Hello Server!" {
                    handle_session_start(self.session_id);
                    ctx.text("{session_id: {}");
                    
                } else {
                    let json_string = handle_position_update(text, self.session_id, self.sessions_storage.clone());
                    ctx.text(json_string);
                }
             } 
            _ => (),
        }
    }
}

pub async fn ws_handler(req: HttpRequest, stream: Payload, data: Data<Arc<Mutex<SessionsStorage>>>) -> Result<HttpResponse, Error> {
    let mut mutable_data = data.lock().unwrap();
    let last_session_id = mutable_data.sessions.len() - 1;
    if last_session_id >= MAX_CONNECTIONS {
        mutable_data.sessions.remove(0);
    }
    mutable_data.sessions.push(Session { positions: vec![Position { x: 0, y: 0 }] }); // make sure to add a new session whenever we create one.
    drop(mutable_data); // required to drop the value here
    println!("last_session_id: {}", last_session_id);
    let resp = ws::start(MyWs {session_id:last_session_id, sessions_storage: data}, &req, stream);
    println!("{:?}", resp);
    resp
}