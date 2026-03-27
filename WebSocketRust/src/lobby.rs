use std::collections::HashMap;

use actix::{Actor, Context, Handler, Message, Recipient};
use uuid::Uuid;

#[derive(Debug)]
#[derive(Message)]
#[rtype(result = "()")]
pub struct WSMessage(pub String);

#[derive(actix::Message)] // This "marks" it as a legal letter to send
#[rtype(result = "()")]    // This says: "The sender doesn't expect a reply"
pub struct Connect {
    pub id: Uuid,
    pub addr: Recipient<WSMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub from: Uuid,
    pub text: String,
}


//Creating a Lobby Struct for the where we map userId and the mailing address of their session Actor
pub struct Lobby{
    sessions : HashMap<Uuid, Recipient<WSMessage>>
}

impl Lobby{
    pub fn new()-> Self{
        Self { sessions: HashMap::new() }
    }
}

impl Actor for Lobby{
    type Context = Context<Self>;
}

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        self.sessions.insert(msg.id, msg.addr);
        log::info!("User {} joined the lobby", msg.id);
    }
}

impl Handler<ClientMessage> for Lobby{
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) -> Self::Result {
        for (id, session) in &self.sessions{
            if *id != msg.from{
                session.do_send(WSMessage(msg.text.clone()));
            }
        }
    }
}

