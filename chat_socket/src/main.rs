use actix::{Actor, ActorContext, Addr, AsyncContext, Handler, StreamHandler, clock::Instant};
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, Responder, get, web};
use actix_web_actors::ws;
use uuid::Uuid;

use crate::{lobby::{Lobby, WSMessage}, protocol::{ChatCommand, Connect}};

mod protocol;
mod lobby;

struct WebSocketSession {
    id: Uuid,
    last_hearbeat: Instant,
    lobby_address: Addr<Lobby>,
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Send a message to the Lobby saying "I'm here!"
        // We send our own address (ctx.address()) so the Lobby can talk back to us.
        self.lobby_address.do_send(Connect {
            id: self.id,
            addr: ctx.address().recipient(),
        });
        log::info!("Actor was born : {}", self.id);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        let duration = self.last_hearbeat.elapsed();
        log::info!("Actor {} died after {}", self.id, duration.as_secs());
    }
}

//Handling incoming data (The "Mailbox")
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(ws::Message::Text(text)) => {
                match serde_json::from_str::<ChatCommand>(&text){
                    Ok(ChatCommand::Join { room, username }) => {
                        log::info!("{} wants to join room {}", username, room);
                        // Logic to tell Lobby to move this user to a room
                    }
                    Ok(ChatCommand::SendMessage { msg }) => {
                        // Logic to broadcast to the room
                    }
                    Ok(ChatCommand::Leave) => ctx.stop(),
                    Err(e) => {
                        log::error!("Invalid JSON received: {}", e);
                        ctx.text("Error: Invalid message format");
                    }
                }

                log::info!("Worker {} received: {}", self.id, text);
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

impl Handler<WSMessage> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: WSMessage, ctx: &mut Self::Context) {
        println!("MESSAGE FROM HANDLER OF WEBSOCKETSESSION IS : {:?}", msg);
        ctx.text(msg.0);
    }
}


#[get("/ws")]
async fn index(
    req: HttpRequest,
    stream: web::Payload,
    web_data: web::Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    // We give every connection a unique ID to track them
    let worker = WebSocketSession {
        id: Uuid::new_v4(),
        last_hearbeat: Instant::now(),
        lobby_address: web_data.get_ref().clone(),
    };
    ws::start(worker, &req, stream)
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let lobby_addr = Lobby::new().start();
    let lobby_data = web::Data::new(lobby_addr);

    log::info!("Starting WebSocket server on 127.0.0.1:8080");
    HttpServer::new(move 
        || App::new().app_data(lobby_data.clone())
        .service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
