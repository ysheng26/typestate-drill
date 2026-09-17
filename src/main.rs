mod actor;
mod session;

use std::thread;

use actor::Client;
use session::Session;

fn typesafe_state_machine() {
    let s = Session::new("broker_url");
    let connecting = s.connect();
    let mut conn = connecting.authenticate();
    conn.publish("hi");
    conn.disconnect();
}

fn main() {
    let client = actor::spawn_client();

    let client_clone = client.clone();

    thread::spawn(move || {
        client_clone.connect();
        client_clone.authenticate();
    });

    std::thread::sleep(std::time::Duration::from_millis(100));
    client.publish("hi");
    client.disconnect();
}
