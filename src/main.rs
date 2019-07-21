//mod io_file;
//mod time;
mod server;
//mod client;

fn main() {
    server::run_server("127.0.0.1", 8080);
}
