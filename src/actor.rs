use std::sync::mpsc;
use std::thread;

use crate::session::Disconnected;

pub enum Command {
    Connect,
    Authenticate,
    Publish(String),
    Disconnect,
}

enum InternalState {
    Disconnected,
    Connecting,
    Ready,
}

#[derive(Clone)]
pub struct Client {
    sender: mpsc::Sender<Command>,
}

impl Client {
    pub fn connect(&self) {
        self.sender.send(Command::Connect).unwrap();
    }

    pub fn authenticate(&self) {
        self.sender.send(Command::Authenticate).unwrap();
    }

    pub fn publish(&self, msg: &str) {
        self.sender
            .send(Command::Publish(String::from(msg)))
            .unwrap();
    }

    pub fn disconnect(&self) {
        self.sender.send(Command::Disconnect).unwrap();
    }
}

pub fn spawn_client() -> Client {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut current_state = InternalState::Disconnected;

        while let Ok(cmd) = rx.recv() {
            match (&current_state, cmd) {
                (InternalState::Disconnected, Command::Connect) => {
                    println!("connecting to broker");
                    current_state = InternalState::Connecting;
                }

                (InternalState::Connecting, Command::Authenticate) => {
                    println!("Authenticating...");
                    current_state = InternalState::Ready;
                }

                (InternalState::Ready, Command::Publish(msg)) => {
                    println!("Publishing message: {}", msg);
                }

                (InternalState::Ready, Command::Disconnect) => {
                    println!("Disconnecting...");
                    current_state = InternalState::Disconnected
                }

                _ => {
                    println!("Invalid command for the current state");
                }
            }
        }

        println!("All clients dropped. Actor shutting down");
    });

    Client { sender: tx }
}
