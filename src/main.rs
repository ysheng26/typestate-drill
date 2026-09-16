mod session;

use session::Session;

fn main() {
    let s = Session::new("broker_url");
    let connecting = s.connect();
    let mut conn = connecting.authenticate();
    conn.publish("hi");
    conn.disconnect();
}
