use std::net::{TcpStream, SocketAddr};
use std::io::{Read, Write};

pub fn run_client(host: &str, port: u16) {
    let s: Vec<&str> = host.split('.').collect();
    let ipv4 = SocketAddr::from(([s[0].parse::<u8>().unwrap(), s[1].parse::<u8>().unwrap(), s[2].parse::<u8>().unwrap(), s[3].parse::<u8>().unwrap()], port));
    let mut stream = TcpStream::connect(ipv4).unwrap();

    stream.write("hola".as_bytes()).unwrap(); 
    println!("{:?}", stream.read(&mut [0; 128])); 
}