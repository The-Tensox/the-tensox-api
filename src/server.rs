use std::rc::Rc;
use std::cell::Cell;
use ws::{listen, Handler, Sender, Result, Message, Handshake, CloseCode, Error};
use std::thread;

pub fn init_server(host: String, port: i16) -> std::thread::JoinHandle<()> {
    thread::spawn(move || {
        let count = Rc::new(Cell::new(0));
        listen(format!("{}:{}", host, port), |out| {
            Server { out: out, count: count.clone() }
        }).unwrap()
    })
}
 
pub struct Server {
    pub out: Sender,
    count: Rc<Cell<u32>>,
}

impl Handler for Server {

    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // Tell the user the current count
        println!("New client, the number of live connections is now {}", self.count.get() + 1);
        // We have a new connection, so we increment the connection counter
        Ok(self.count.set(self.count.get() + 1))
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
            CloseCode::Away   => println!("The client is leaving the site."),
            CloseCode::Abnormal => println!(
                "Closing handshake failed! Unable to obtain closing status from client."),
            _ => println!("The client encountered an error: {}", reason),
        }

        // The connection is going down, so we need to decrement the count
        self.count.set(self.count.get() - 1)
    }

    fn on_error(&mut self, err: Error) {
        println!("The server encountered an error: {:?}", err);
    }

}


/*
use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use std::env;
use std::ops::Deref;


impl<'a, 'r> FromRequest<'a, 'r> for Server {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Server, Self::Error> {
        Outcome::Success(())
    }
}

*/