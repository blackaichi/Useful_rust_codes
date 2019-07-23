use std::net::{TcpListener, TcpStream, SocketAddr, IpAddr, Ipv4Addr};
use std::io::{Read, Write};
use std::thread;

// This function creates a TCP server with a given IPv4 and a port
pub fn run_tcp_server(host: &str, port: u16) {
    
    if is_ipv4(host) {
        let s: Vec<&str> = host.split('.').collect();
        // creating a socket address with the ipv4 and 
        let ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(s[0].parse::<u8>().unwrap(), s[1].parse::<u8>().unwrap(),
                                                          s[2].parse::<u8>().unwrap(), s[3].parse::<u8>().unwrap())), port);
        run_server(ip);
    }
    // Not implemented yet
    else {
        panic!("Ipv6 not implemented yet");
        /*
        let s: Vec<&str> = host.split(':').collect();
        // creating a socket address with the ipv4 and 
        let ip = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(s[0].parse::<u16>().unwrap(), s[1].parse::<u16>().unwrap(),
                                                          s[2].parse::<u16>().unwrap(), s[3].parse::<u16>().unwrap(),
                                                          s[4].parse::<u16>().unwrap(), s[5].parse::<u16>().unwrap(),
                                                          s[6].parse::<u16>().unwrap(), s[7].parse::<u16>().unwrap())), port);
        run_server(ip);
        */
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

fn is_ipv4(ip: &str) -> bool {
    if ip.contains('.') {
        let a: Vec<&str> = ip.split('.').collect();
        if a.len() != 4 {
            return false;
        }
        for n in a {
            let x = n.parse::<i32>().unwrap();
            if x > 255 || x < 0 {
                return false;
            }
        }
        return true;
    }
    if ip.contains(':') {
        panic!("unvalid address, aborting program"); 
        /* UNIMPLEMENTED YET
        let a: Vec<&str> = ip.split(':').collect();
        if a.len() != 8 {
            return false;
        }
        for n in a {
            
            }
        }
        true
        */
    }
    panic!("unvalid address, aborting program");
}

fn run_server(ip: SocketAddr) {

    let listener = TcpListener::bind(ip).unwrap();

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