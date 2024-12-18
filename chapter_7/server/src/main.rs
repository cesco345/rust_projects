use common::{ConnectionStatus, Message, MessageHandler};

struct Server {
    clients: Vec<ConnectionStatus>,
}

impl Server {
    fn new() -> Self {
        Server {
            clients: Vec::new(),
        }
    }

    fn start(&mut self) {
        println!("Server started");
    }
}

impl MessageHandler for Server {
    fn handle_message(&mut self, message: Message) {
        match message {
            Message::Text(text) => println!("Broadcasting: {}", text),
            Message::Status(status) => {
                self.clients.push(status);
                println!("Client status updated");
            }
        }
    }
}

fn main() {
    let mut server = Server::new();
    server.start();
    server.handle_message(Message::Text("Server broadcast".to_string()));
}
