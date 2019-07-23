//mod io_file;
//mod time;
mod server;
mod client;
use std::env;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();

    let arg = args.clone();

    let handle = thread::spawn(move || {
        server::run_tcp_server(&args[1].to_string(), args[2].parse::<u16>().unwrap());
    });

    thread::spawn(move || {
        client::run_client(&arg[1].to_string(), arg[2].parse::<u16>().unwrap())
    });

    handle.join().unwrap();
}
