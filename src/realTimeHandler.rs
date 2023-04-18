use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

pub async fn Ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let res = ws::start(MyWebSocket {}, &r, stream);
    println!("{:?}", res);
    res
}

struct MyWebSocket {}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            // Ping/Pong will be used to make sure the connection is still alive
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {}
            // Text will echo any text received back to the client (for now)
            Ok(ws::Message::Text(text)) => ctx.text(text),
            // Close will close the socket
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}