use std::env;
use std::i16;
use std::string::String;
use std::str::FromStr;

extern crate getopts;

use getopts::Options;

pub mod server;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} -p PORT -i INTERFACE", program);
    print!("{}", opts.usage(&brief));
}

fn parse_args() -> (Option<String>, Option<i16>) {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("i", "interface", "set server interface", "INTERFACE");
    opts.optopt("p", "port", "set server port", "PORT");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return (None, None);
    }
    let interface = matches.opt_str("i");
    let port = matches.opt_str("p");

    if port == None || interface == None {
        print_usage(&program, opts);
        return (None, None);
    }

    let port_final = match i16::from_str(port.unwrap().as_str().as_ref()) {
        Ok(v) => v,
        Err(e) => {
            println!("Arguments error: {}", e);
            print_usage(&program, opts);
            return (None, None);
        }
    };
    return (interface, Option::from(port_final));
}

fn main() {
    let (interface, port) = parse_args();
    if interface == None || port == None {
        return;
    }

    let server = server::tcp_server::TCPServer::new(interface.unwrap(), port.unwrap());
    let server_thread = server.start();
    server_thread.join().expect("Joining server thread failed");
}
