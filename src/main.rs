pub mod server;
pub mod util;

use std::env;
use util::arguments::ArgumentParser;
use server::tcp_server::TCPServer;

fn main() {
    let argument_parser = ArgumentParser::new(env::args().collect());
    let (interface,
        port,
        device,
        baud) = argument_parser.parse();
    if interface == None || port == None || device == None || baud == None {
        return;
    }

    let server = TCPServer::new(interface.unwrap(), port.unwrap());
    let server_thread = server.start();
    server_thread.join().expect("Joining server thread failed");
}
