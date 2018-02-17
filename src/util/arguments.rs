extern crate getopts;

use self::getopts::Options;
use std::{i16, usize};
use std::string::String;
use std::str::FromStr;

pub struct ArgumentParser {
    args: Vec<String>
}

impl ArgumentParser {
    pub fn new(args: Vec<String>) -> ArgumentParser {
        ArgumentParser { args }
    }

    pub fn parse(&self) -> (Option<String>, Option<i16>, Option<String>, Option<usize>, Option<bool>, Option<usize>) {
        let program = self.args[0].clone();

        let mut opts = Options::new();
        opts.optflag("h", "help", "print this help menu");
        opts.optflag("v", "version", "print program version");
        opts.optflag("s", "skip", "skip connection check");
        opts.optopt("i", "interface", "set server interface", "INTERFACE");
        opts.optopt("p", "port", "set server port", "PORT");
        opts.optopt("d", "device", "set serial device", "DEVICE");
        opts.optopt("b", "baud", "set serial baud rate", "BAUD");
        opts.optopt("t", "thread", "set thread pool size", "THREADS");
        let matches = match opts.parse(&self.args[1..]) {
            Ok(m) => { m }
            Err(f) => { panic!(f.to_string()) }
        };
        let mut _skip_check = false;
        if matches.opt_present("h") {
            ArgumentParser::print_usage(&program, opts);
            return (None, None, None, None, None, None);
        }
        if matches.opt_present("v") {
            ArgumentParser::print_version(&program);
            return (None, None, None, None, None, None);
        }
        if matches.opt_present("s") {
            _skip_check = true;
        }
        let interface = matches.opt_str("i");
        let port = matches.opt_str("p");
        let device = matches.opt_str("d");
        let baud = matches.opt_str("b");
        let threads = matches.opt_str("t");

        if port == None || interface == None || device == None || baud == None {
            ArgumentParser::print_usage(&program, opts);
            return (None, None, None, None, None, None);
        }

        let port_number = match i16::from_str(port.unwrap().as_str().as_ref()) {
            Ok(v) => v,
            Err(e) => {
                println!("Invalid port: {}", e);
                ArgumentParser::print_usage(&program, opts);
                return (None, None, None, None, None, None);
            }
        };
        let baud_number = match usize::from_str(baud.unwrap().as_str().as_ref()) {
            Ok(v) => v,
            Err(e) => {
                println!("Invalid baud: {}", e);
                ArgumentParser::print_usage(&program, opts);
                return (None, None, None, None, None, None);
            }
        };
        let pool_size = match usize::from_str(threads.unwrap_or(String::from("4")).as_str().as_ref()) {
            Ok(v) => {
                if v < 1 {
                    println!("Thread pool size must be greater than 0");
                    ArgumentParser::print_usage(&program, opts);
                    return (None, None, None, None, None, None);
                }
                v
            },
            Err(e) => {
                println!("Invalid thread pool size: {}", e);
                ArgumentParser::print_usage(&program, opts);
                return (None, None, None, None, None, None);
            }
        };
        return (interface, Option::from(port_number), device, Option::from(baud_number),
                Option::from(_skip_check), Option::from(pool_size));
    }

    fn print_usage(program: &str, opts: Options) {
        let brief = format!("Usage: {} -p PORT -i INTERFACE -d DEVICE -b BAUD [-t THREADS] [-v|-h|-s]", program);
        print!("{}", opts.usage(&brief));
    }

    fn print_version(program: &str) {
        println!("{} {}", program, env!("CARGO_PKG_VERSION"));
    }
}