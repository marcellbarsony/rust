use std::io::{Read, Write};

use std::net::{
    IpAddr,
    Ipv4Addr,
    SocketAddr,
    TcpListener,
    TcpStream
};

fn main() {
    // Set IPv4
    let localhost_v4 = IpAddr::V4(
        Ipv4Addr::new(127, 0, 0, 1)
    );

    // Set port
    let port = 1234;

    // Create socket address
    let socket = SocketAddr::new(localhost_v4, port);

    // Bind socket address
    let listener = TcpListener::bind(socket)
        .expect(":: [-] :: Bind socket");

    // Listen to incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(err) => {
                eprintln!(":: [-] :: Get stream :: {err}");
            }
        }

    }
}

fn handle_connection(mut stream: TcpStream) {
    // Create buffer
    let mut buffer = [0; 1024];

    // Pull stream bytes into buffer
    match stream.read(&mut buffer){
        Ok(bytes_read) => {
            if bytes_read == 0 {
                println!(":: [-] :: Connection closed by peer");
            } else {
                println!(
                    ":: [i] :: Incoming data :: {:?}",
                    String::from_utf8_lossy(&buffer[..bytes_read])
                );
            }
        }
        Err(err) => {
            eprintln!(":: [-] :: Read stream :: {err}");
        }
    }

    // Create response
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    // Write response to TcpStream
    if let Err(err) = stream.write(response.as_bytes()) {
        eprintln!(":: [-] :: Write stream :: {err}");
    }

    // Flush output stream
    stream.flush().expect(":: [-] :: Flush stream");
}
