use std::collections::HashMap;

use actix::{Actor, Context, Handler, Message, Recipient};
use uuid::Uuid;

use crate::protocol::{ClientMessage, Connect, Disconnect};

#[derive(Debug)]
#[derive(Message)]
#[rtype(result = "()")]
pub struct WSMessage(pub String);



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

impl Handler<Disconnect> for Lobby {
    type Result = ();
    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        if self.sessions.remove(&msg.id).is_some() {
            log::info!("Lobby: User {} removed. Total: {}", msg.id, self.sessions.len());
        }
    }
}

