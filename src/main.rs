pub mod server;
pub mod util;

use std::env;
use util::arguments::ArgumentParser;
use util::serial_stream::SerialStream;
use server::tcp_server::TCPServer;

fn main() {
    let (interface,
        port,
        device,
        baud) = ArgumentParser::new(env::args().collect()).parse();
    if interface == None || port == None || device == None || baud == None {
        return;
    }

    let serial_stream = SerialStream::new(device.unwrap(), baud.unwrap());
    serial_stream.open();

    let server = TCPServer::new(interface.unwrap(), port.unwrap());
    let server_thread = server.start();

    server_thread.join().expect("Joining server thread failed");
}
