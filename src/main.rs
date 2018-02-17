pub mod command;
pub mod server;
pub mod util;

use std::env;
use std::time::Instant;
use command::executor::Executor;
use util::arguments::ArgumentParser;
use util::serial_stream::SerialStream;
use server::tcp_server::TCPServer;

fn main() {
    let time = Instant::now();
    let (interface,
        port,
        device,
        baud,
        skip_check,
        pool_size) = ArgumentParser::new(env::args().collect()).parse();
    if interface == None || port == None || device == None || baud == None {
        return;
    }

    let mut server = TCPServer::new(interface.clone().unwrap(), port.clone().unwrap(),
                                    pool_size.clone().unwrap(), device.clone().unwrap(),
                                    baud.clone().unwrap());
    let server_thread = server.start();

    println!("TCP server started on {}:{}", interface.clone().unwrap(), port.clone().unwrap());
    println!("Thread pool size {}", pool_size.clone().unwrap());
    println!("Serial stream on {} with baud {}", device.clone().unwrap(), baud.clone().unwrap());

    if !skip_check.unwrap() {
        let mut serial_stream = SerialStream::new(device.clone().unwrap(),
                                                  baud.clone().unwrap());
        serial_stream.open();
        let check_resp = Executor::run(&mut vec![0x31], &mut serial_stream);
        println!("Serial connection check: {:?}", if check_resp.success { "OK" } else { "FAIL" });
        if !check_resp.success {
            panic!("Serial connection failed");
        }
    }

    let elapsed = time.elapsed();
    println!("Initialization finished in {}ms",
             (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);

    println!();
    server_thread.join().expect("Joining server thread failed");
}
