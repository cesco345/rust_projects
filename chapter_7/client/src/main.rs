use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Text(String),
    Status(ConnectionStatus),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
    Error(String),
}

pub trait MessageHandler {
    fn handle_message(&mut self, message: Message);
}
