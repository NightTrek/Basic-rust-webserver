use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use serde_json;

// q: "how do i import serde to the cargo.toml?"
// a: "add serde = { version = "1.0", features = ["derive"] } to the dependencies section of your Cargo.toml"
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Coordinates {
    x: f32,
    y: f32,
}

struct MyWebSocket;

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("WebSocket started");
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                // Deserialize the coordinates sent by the client
                let coords: Coordinates = serde_json::from_str(&text).unwrap();

                // Process the coordinates
                let new_coords = Coordinates {
                    x: coords.x + 1.0,
                    y: coords.y + 1.0,
                };

                // Send the new coordinates back to the client
                ctx.text(serde_json::to_string(&new_coords).unwrap());
            }
            _ => (),
        }
    }
}

pub async fn ws_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWebSocket {}, &req, stream);
    resp
}