use std::net::{TcpListener, TcpStream, Ipv4Addr, SocketAddr};
use std::io::{ Read, Write};
use std::thread;

pub fn run_server(host: &str, port: u16) {
    let s: Vec<&str> = host.split('.').collect();
    let ipv4 = SocketAddr::from(([s[0].parse::<u8>().unwrap(), s[1].parse::<u8>().unwrap(), s[2].parse::<u8>().unwrap(), s[3].parse::<u8>().unwrap()], port));
    let listener = TcpListener::bind(ipv4).unwrap();
    println!("Listening for connections on port {}", 8080);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_connection(stream)
                });
            },
            Err(e) => println!("Can't connect to the server: {:?}", e),
        }
    }
}

fn handle_connection(stream: TcpStream) {
    stream_read(&stream);
    stream_write(&stream);
}

fn stream_read(mut stream: &TcpStream) {
    let mut buff: [u8; 4096] = [0; 4096];

    match stream.read(&mut buff) {
        Ok(_) => println!("{}", String::from_utf8_lossy(&buff)),
        Err(e) => println!("Failed to read stream: {}", e),
    }
}

fn stream_write(mut stream: &TcpStream) {
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
    match stream.write(response) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}