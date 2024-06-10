use std::{net::TcpListener, thread::spawn};
use tungstenite::{
    accept,
    handshake::server::{Request, Response},
};

// pub struct

pub fn run() {
    let server = TcpListener::bind("127.0.0.1:3012").unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();

            loop {
                let msg = websocket.read().unwrap();
                // respond to requests
            }
        });
    }
}
