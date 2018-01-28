use std::io::{Read, Write, BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};
use std::vec::Vec;
use std::string::String;
use std::str;
use std::thread;
use std::thread::JoinHandle;

pub struct TCPServer {
    interface: String,
    port: i16,
}

impl TCPServer {
    pub fn new(interface: String, port: i16) -> TCPServer {
        TCPServer { interface, port }
    }

    pub fn start(&self) -> JoinHandle<()> {
        let address = format!("{}:{}", self.interface, self.port.to_string());
        let listener = TcpListener::bind(address).expect("Unable to bind TCP server");
        println!("Server is ready to accept TCP connections");

        let server_thread = thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        thread::spawn(move || {
                            TCPServer::handle_client(stream);
                        });
                    }
                    Err(e) => panic!("Connection failed: {}", e)
                }
            }
        });
        return server_thread;
    }

    fn handle_client(stream: TcpStream) {
        let mut writer = BufWriter::new(&stream);
        writer.write_all(b"Hello user\r\n").expect("Error writing");
        writer.flush().expect("Could not flush");

        let mut reader = BufReader::new(&stream);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).expect("Error reading");
        let response = match str::from_utf8(buffer.as_ref()) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {error}", error = e),
        };
        println!("Received: {}", response);
    }
}