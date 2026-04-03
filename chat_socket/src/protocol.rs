use actix::{Message, Recipient};
use serde::*;
use uuid::Uuid;

use crate::lobby::WSMessage;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ChatCommand{
    Join { room : String, username : String},
    SendMessage { msg : String},
    Leave
}

#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
}


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