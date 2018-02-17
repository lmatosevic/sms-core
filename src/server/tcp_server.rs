use std::io::{Write, BufReader, BufRead, BufWriter};
use std::net::{TcpListener, TcpStream};
use std::vec::Vec;
use std::string::String;
use std::thread;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use command::executor::Executor;
use util::serial_stream::SerialStream;
use server::thread_pool::ThreadPool;

pub struct TCPServer {
    interface: String,
    port: i16,
    pool_size: usize,
    device: String,
    baud: usize,
}

impl TCPServer {
    pub fn new(interface: String, port: i16, pool_size: usize, device: String, baud: usize) -> TCPServer {
        TCPServer { interface, port, pool_size, device, baud }
    }

    pub fn start(&mut self) -> JoinHandle<()> {
        let address = format!("{}:{}", self.interface, self.port.to_string());
        let listener = TcpListener::bind(address).expect("Unable to bind TCP server");
        let pool = ThreadPool::new(self.pool_size.clone());

        let device = self.device.clone();
        let baud = self.baud.clone();

        let server_thread = thread::spawn(move || {
            let mut serial_stream = SerialStream::new(device, baud);
            serial_stream.open();
            let mutex = Arc::new(Mutex::new(serial_stream));
            for stream in listener.incoming() {
                let mutex_c = mutex.clone();
                match stream {
                    Ok(stream) => {
                        let peer_addr = stream.peer_addr().unwrap();
                        pool.execute(move || {
                            println!("Accepted connection from {}", peer_addr);
                            TCPServer::handle_client(stream, mutex_c);
                            println!("Closed connection from {}", peer_addr);
                        });
                    }
                    Err(e) => panic!("Connection failed: {}", e)
                }
            }
            drop(pool);
        });

        return server_thread;
    }

    fn handle_client(stream: TcpStream, mutex: Arc<Mutex<SerialStream>>) {
        let mut b = [0; 1];
        while stream.peek(&mut b).is_ok() {
            let mut reader = BufReader::new(&stream);
            let mut buffer: Vec<u8> = Vec::new();
            let _size = reader.read_until(0x04 as u8, &mut buffer).expect("Error reading from socket");
            let _ = buffer.pop(); // Remove 0x04 - end of transaction byte

            println!("Received: {}", String::from_utf8(buffer.clone()).unwrap());

            // Mutex locks the critical segment - serial port read & write only one thread at the time
            let mut serial_guard = match mutex.lock() {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner() // Recover from mutex poisoning
            };
            let response = Executor::run(&mut buffer, &mut serial_guard);
            drop(serial_guard);

            let result = String::from_utf8(response.data).unwrap();

            println!("Sent: {}", result);

            let mut writer = BufWriter::new(&stream);
            writer.write_all(format!("{}\n", result).as_bytes()).expect("Error writing to socket");
            writer.flush().expect("Could not flush");
        }
    }
}