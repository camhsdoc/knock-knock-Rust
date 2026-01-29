use std::{
    env,
    net::{UdpSocket, SocketAddr},
    time::Duration,
    io,
};

fn main() -> Result<(), io::Error> {
    // 1. Argument Parsing
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("USAGE: KNOCK <IP_ADDRESS>");
        eprintln!("Example: KNOCK 192.168.1.1");
        return Ok(());
    }

    // 2. Setup
    let payload = b"KNOCK KNOCK";
    let host = &args[1];
    let port = "64800";
    let destination = format!("{}:{}", host, port);
    
    println!("Sending KNOCK to {} with payload: {:?}", destination, payload);

    // 3. Socket Binding (Binding to 0.0.0.0:0 lets the OS choose a port)
    let udpsocket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: failed to bind socket: {}", e);
            return Err(e);
        }
    };

    // 4. Sending Data
    let destination_addr: SocketAddr = match destination.parse() {
        Ok(addr) => addr,
        Err(e) => {
            eprintln!("Error: Invalid destination address format (must be IP:Port): {}", e);
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid address format"));
        }
    };

    match udpsocket.send_to(payload, destination_addr) {
        Ok(bytes_sent) => println!("Sent {} bytes to {}", bytes_sent, destination),
        Err(e) => {
            eprintln!("Error: failed to send data to {}: {}", destination, e);
            return Err(e);
        }
    };

    // 5. Handle Reply
    show_reply(udpsocket)?;
    
    Ok(())
}

fn show_reply(socket: UdpSocket) -> Result<(), io::Error> {
    let mut buf = [0u8; 1500]; // Buffer for received data
    
    // Set a read timeout
    match socket.set_read_timeout(Some(Duration::from_secs(2))) {
        Ok(_) => println!("Waiting for reply (2s timeout)..."),
        Err(e) => {
            eprintln!("Warning: failed to set timeout: {}", e);
            // Continue execution, but with a warning
        }
    }

    // Attempt to receive data
    match socket.recv_from(&mut buf) {
        Ok((len, src)) => {
            // Try to convert the received bytes to a UTF-8 string
            match std::str::from_utf8(&buf[..len]) {
                Ok(line) => {
                    println!("\nReply from {}:", src);
                    println!("{}", line);
                }
                Err(e) => {
                    eprintln!("Error: Received message but unable to interpret as UTF-8: {}", e);
                    // Print the raw bytes for debugging
                    println!("Raw data: {:?}", &buf[..len]);
                }
            }
        }
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock || e.kind() == io::ErrorKind::TimedOut => {
            // This is the expected error for a timeout
            println!("\nNo answer! (Timeout occurred after 2 seconds)");
        }
        Err(e) => {
            // Handle all other socket errors
            eprintln!("\nNetwork error during receive: {}", e);
            return Err(e);
        }
    }

    Ok(())
}