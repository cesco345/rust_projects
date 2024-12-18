use common::{ConnectionStatus, Message, MessageHandler};

struct Client {
    connection_status: ConnectionStatus,
}

impl Client {
    fn new() -> Self {
        Client {
            connection_status: ConnectionStatus::Disconnected,
        }
    }

    fn connect(&mut self) {
        self.connection_status = ConnectionStatus::Connected;
        println!("Client connected");
    }
}

impl MessageHandler for Client {
    fn handle_message(&mut self, message: Message) {
        match message {
            Message::Text(text) => println!("Received: {}", text),
            Message::Status(status) => {
                self.connection_status = status;
                println!("Connection status changed: {:?}", self.connection_status);
            }
        }
    }
}

fn main() {
    let mut client = Client::new();
    client.connect();
    client.handle_message(Message::Text("Hello from client!".to_string()));
}
