use std::io::BufRead;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;

use tracing::debug;
use tracing::info;
use tracing::span;
use tracing::subscriber::set_global_default;
use tracing::warn;
use tracing::Level;

pub fn handle_client(stream: TcpStream) {
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        debug!("{}", line.unwrap());
    }
    warn!("Remote host closed connection.");
}

fn main() -> std::io::Result<()> {
    let collector = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .compact()
        .finish();
    set_global_default(collector).unwrap();

    // Easy to connect to the socket. Tail the file and send it over the socket
    // `journalctl -f | nc localhost 3000`
    let listener = TcpListener::bind("0.0.0.0:3000")?;
    info!("Listening for connections");

    for stream in listener.incoming() {
        let stream = stream?;
        let peer_addr = stream.peer_addr().unwrap();
        let connected_span = span!(Level::TRACE, "connection");
        info!("Accepting connection from {}", peer_addr);
        spawn(|| {
            handle_client(stream);
        });
        let _guard = connected_span.enter();
    }
    Ok(())
}
