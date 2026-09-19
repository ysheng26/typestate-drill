use std::sync::mpsc;
use std::thread;

use crate::session::{self, Connecting, Disconnected, Ready, Session};

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

enum ActiveSession {
    Disconnected(Session<Disconnected>),
    Connecting(Session<Connecting>),
    Ready(Session<Ready>),
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

pub fn spawn_client_typestate() -> Client {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let initial_session = Session::new("broker_url");
        let mut current_state = Some(ActiveSession::Disconnected(initial_session));

        while let Ok(cmd) = rx.recv() {
            let state = current_state.take().unwrap();

            current_state = Some(match (state, cmd) {
                (ActiveSession::Disconnected(session), Command::Connect) => {
                    println!("Connecting to broker...");
                    let next_session = session.connect();
                    ActiveSession::Connecting(next_session)
                }
                (ActiveSession::Connecting(session), Command::Authenticate) => {
                    println!("Authenticating...");
                    let next_session = session.authenticate();
                    ActiveSession::Ready(next_session)
                }
                (ActiveSession::Ready(mut session), Command::Publish(msg)) => {
                    session.publish(&msg);
                    ActiveSession::Ready(session)
                }
                (ActiveSession::Ready(session), Command::Disconnect) => {
                    println!("Disconnecting...");
                    let next_session = session.disconnect();
                    ActiveSession::Disconnected(next_session)
                }
                (original_state, bad_cmd) => {
                    println!("Invalid command");
                    original_state
                }
            });
        }
        println!("All clients dropped. Actor shutting down");
    });
    Client { sender: tx }
}
