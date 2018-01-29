pub mod command;
pub mod server;
pub mod util;

use std::env;
use std::time::Instant;
use util::arguments::ArgumentParser;
use util::serial_stream::SerialStream;
use server::tcp_server::TCPServer;

fn main() {
    let time = Instant::now();
    let (interface,
        port,
        device,
        baud) = ArgumentParser::new(env::args().collect()).parse();
    if interface == None || port == None || device == None || baud == None {
        return;
    }

    let mut serial_stream = SerialStream::new(device.unwrap(), baud.unwrap());
    serial_stream.open();

    let server = TCPServer::new(interface.unwrap(), port.unwrap());
    let server_thread = server.start();

    let elapsed = time.elapsed();
    println!("Server started in {} ms",
             (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);

    server_thread.join().expect("Joining server thread failed");
}
