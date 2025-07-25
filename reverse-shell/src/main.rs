use std::io::{stdin, Read, Write};

use std::net::{
    IpAddr,
    Ipv4Addr,
    SocketAddr,
    TcpListener,
    TcpStream
};
use std::thread;

fn main() {
    // Listening IP
    let localhost_v4 = IpAddr::V4(
        Ipv4Addr::new(127, 0, 0, 1)
    );

    // Listening port
    let port = 1234;

    // Create socket
    let socket = SocketAddr::new(localhost_v4, port);

    // Bind socket to listener
    let listener = TcpListener::bind(socket)
        .expect(":: [-] :: Bind socket");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_connection(stream));
            }
            Err(err) => {
                eprintln!(":: [-] :: Print error :: {err}");
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    loop {
        // Create buffer
        let mut buffer = [0; 1024];

        // Pull stream bytes into buffer
        match stream.read(&mut buffer){
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!(":: [-] :: Connection closed by peer");
                } else {
                    println!(
                        ":: [i] :: Incoming data :: \n{}",
                        String::from_utf8_lossy(&buffer[..bytes_read])
                    );
                }
            }
            Err(err) => {
                eprintln!(":: [-] :: Read stream :: {err}");
            }
        }

        // Get input
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect(":: [-] :: Cannot read input");

        stream.write_all(input.as_bytes())
            .expect(":: [-] :: Failed to write to stream")
    }
}
