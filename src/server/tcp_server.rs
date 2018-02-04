use std::io::{Write, BufReader, BufRead, BufWriter};
use std::net::{TcpListener, TcpStream};
use std::vec::Vec;
use std::string::String;
use std::thread;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use command::executor::Executor;
use util::serial_stream::SerialStream;

pub struct TCPServer {
    interface: String,
    port: i16,
    device: String,
    baud: usize,
}

impl TCPServer {
    pub fn new(interface: String, port: i16, device: String, baud: usize) -> TCPServer {
        TCPServer { interface, port, device, baud }
    }

    pub fn start(&mut self) -> JoinHandle<()> {
        let address = format!("{}:{}", self.interface, self.port.to_string());
        let listener = TcpListener::bind(address).expect("Unable to bind TCP server");

        let device = self.device.clone();
        let baud = self.baud.clone();
        let mutex = Arc::new(Mutex::new(0));

        let server_thread = thread::spawn(move || {
            for stream in listener.incoming() {
                let mutex_c = mutex.clone();
                let device_ref = device.clone(); // Clone serial conf for every thread in loop
                let baud_ref = baud.clone();
                let mut serial_stream = SerialStream::new(device_ref, baud_ref);
                match stream {
                    Ok(stream) => {
                        thread::spawn(move || {
                            TCPServer::handle_client(stream, serial_stream, mutex_c);
                        });
                    }
                    Err(e) => panic!("Connection failed: {}", e)
                }
            }
        });

        return server_thread;
    }

    fn handle_client(stream: TcpStream, mut serial_stream: SerialStream, mutex: Arc<Mutex<i32>>) {
        let mut reader = BufReader::new(&stream);
        let mut buffer: Vec<u8> = Vec::new();
        let _size = reader.read_until(0x04 as u8, &mut buffer).expect("Error reading from socket");
        let _ = buffer.pop(); // Remove 0x04 - end of transaction byte

        println!("Received: {}", String::from_utf8(buffer.clone()).unwrap());

        // Mutex locks the critical segment - serial port read & write only one thread at the time
        let guard = match mutex.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner() // Recover from mutex poisoning
        };
        let response = Executor::run(&mut buffer, &mut serial_stream);
        drop(guard);

        let result = String::from_utf8(response.data).unwrap();

        println!("Sent: {}", result);

        let mut writer = BufWriter::new(&stream);
        writer.write_all(format!("{}\n", result).as_bytes()).expect("Error writing to socket");
        writer.flush().expect("Could not flush");
    }
}