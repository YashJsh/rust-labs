use serde::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ChatCommand{
    Join { room : String, username : String},
    SendMessage { msg : String},
    Leave
}