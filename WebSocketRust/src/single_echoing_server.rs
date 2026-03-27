use actix::{Actor, ActorContext, Addr, AsyncContext, StreamHandler, WrapFuture, clock::Instant, dev::Stream, fut};
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use uuid::Uuid;

struct WebSocketSession{
    id : Uuid,
    last_hearbeat  : Instant,
}

impl Actor for WebSocketSession{
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::info!("Actor was born : {}", self.id);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        let duration = self.last_hearbeat.elapsed();
        log::info!("Actor {} died after {}", self.id,duration.as_secs());
    }
}

//Handling incoming data (The "Mailbox")
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession{
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item{
            Ok(ws::Message::Text(text))=> {
                log::info!("Worker {} received: {}", self.id, text);
                ctx.text(format!("Echo: {}", text)); //Echoing the message
            },
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop(); // This triggers the 'stopped' hook
            }
            _ => (),
        }
    }
}

#[get("/ws")]
async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    // We give every connection a unique ID to track them
    let worker = WebSocketSession { 
        id: Uuid::new_v4(), 
        last_hearbeat: Instant::now() 
    };
    ws::start(worker, &req, stream)
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    log::info!("Starting WebSocket server on 127.0.0.1:8080");
    HttpServer::new(
        move || 
        App::new()
        .service(index)
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
