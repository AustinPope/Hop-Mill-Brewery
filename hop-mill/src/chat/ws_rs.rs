// Websocket server from guide https://github.com/steadylearner/Chat
use ws::{
    listen, CloseCode, Error, Handler, Handshake, Message, Request, Response, Result, Sender,
};

use std::cell::Cell;
use std::rc::Rc;

// Server web app handler
struct Server {
    out: Sender,
    count: Rc<Cell<u32>>,
}

impl Handler for Server {
    fn on_request(&mut self, req: &Request) -> Result<Response> {
        match req.resource() {
            // used once const socket = new WebSocket("ws://" + window.location.host + "/ws");
            "/ws" => {
                // https://ws-rs.org/api_docs/ws/struct.Request.html
                println!("Browser Request from {:?}", req.origin().unwrap().unwrap());
                println!("Client found is {:?}", req.client_addr().unwrap());

                let resp = Response::from_request(req);
                resp
            }
            _ => Ok(Response::new(
                404,
                "Oops! Not Found.",
                b"404 - Not Found".to_vec(),
            )),
        }
    }

    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        // new connection, increment connection counter
        self.count.set(self.count.get() + 1);
        let num_connections = self.count.get();

        // handled in chat/bundler.js
        // if num_connections > 5 {
        //     panic!("Oh snap! There are more users connected than expected!");
        // }

        // used to assign id for clients
        let open_message = format!(
            "{} entered and the number of live connections is {}",
            &handshake.peer_addr.unwrap(),
            &num_connections
        );

        println!("{}", &open_message);
        self.out.broadcast(open_message);

        Ok(())
    }

    // Handle messages received in the websocket (/ws)
    fn on_message(&mut self, message: Message) -> Result<()> {
        let raw_message = message.into_text()?;
        println!("The message from the client is {:#?}", &raw_message);

        let message = if raw_message.contains("!warn") {
            let warn_message = "One of the clients set warning to the server.";
            println!("{}", &warn_message);
            Message::Text("There was a warning from another user.".to_string())
        } else {
            Message::Text(raw_message)
        };

        self.out.broadcast(message)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away => println!("The client is leaving the site."),
            CloseCode::Abnormal => {
                println!("Closing handshake failed! Unable to obtain closing status from client.")
            }
            _ => println!("The client encountered an error: {}", reason),
        }
        self.count.set(self.count.get() - 1)
    }

    fn on_error(&mut self, err: Error) {
        println!("The server encountered an error: {:?}", err);
    }
}

pub fn websocket() -> () {
    println!("Web Socket Server is ready at ws://0.0.0.0:7777/ws");
    println!("Server is ready at http://0.0.0.0:7777/");

    // Rc is a reference-counted box for sharing the count between handlers
    // each handler needs its own contents.
    // Cell gives us interior mutability so we can increment/decrement count
    // between handlers.

    // Listen on an address and call the closure for each connection
    let count = Rc::new(Cell::new(0));
    listen("0.0.0.0:7777", |out| Server {
        out: out,
        count: count.clone(),
    })
    .unwrap()
}
