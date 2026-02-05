use std::str;
use std::net::{SocketAddr, UdpSocket};

fn main() {
    // Bind UDP socket and listen for knocks
    let socket = UdpSocket::bind("0.0.0.0:64800")
        .expect("Unable to open listener.");

    println!("WAITING FOR KNOCKS");

    loop {
        let mut buf = [0u8; 1500];

        match socket.recv_from(&mut buf) {
            Ok((_, source)) => {
                match str::from_utf8(&buf) {
                    Ok(payload) => {
                        println!("{} from {}", payload.trim_matches(char::from(0)), source);
                        whos_there(&socket, source);
                    }
                    Err(e) => {
                        println!("String error: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Network error: {}", e);
            }
        }
    }
}

fn whos_there(socket: &UdpSocket, dest: SocketAddr) {
    let reply = b"WHO'S THERE?";

    match socket.send_to(reply, dest) {
        Ok(_) => println!("Sent reply to {}", dest),
        Err(e) => println!("Send error: {}", e),
    }
}
