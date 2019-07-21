use std::net::TcpStream;

pub fn run_client(ip: &str) {
    let mut stream = TcpStream::connect(ip).unwrap();

    stream.write("hola"); 
    println!("{:?}", stream.read(&mut [0; 128])); 
}