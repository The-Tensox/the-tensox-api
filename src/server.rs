use ws::{Handler, Sender, Result, Message, Handshake, CloseCode, Error};

#[derive(Debug)]
pub struct Server {
    pub out: Option<Sender>,
}

impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // Tell the user the current count
        println!("New client");
        // We have a new connection, so we increment the connection counter
        Ok(())
    }

    fn on_message(&mut self, _: Message) -> Result<()> {
        // Echo the message back
        // self.out.send(msg)
        // println!("Received {}", msg);
        Ok(())
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
    }

    fn on_error(&mut self, err: Error) {
        println!("The server encountered an error: {:?}", err);
    }
}
