extern crate serial;

use self::serial::BaudRate;
use self::serial::SystemPort;
use self::serial::prelude::*;
use std::time::Duration;
use std::vec::Vec;
use std::io;
use std::io::prelude::*;

pub struct SerialStream {
    device: String,
    baud: usize,
    port: Option<SystemPort>,
}

impl SerialStream {
    pub fn new(device: String, baud: usize) -> SerialStream {
        SerialStream { device, baud, port: None }
    }

    pub fn open(&mut self) {
        let mut port = serial::open(&self.device).expect("Unable to open serial port");
        port.reconfigure(&|settings| {
            let _ = settings.set_baud_rate(SerialStream::baud_to_enum(self.baud));
            settings.set_char_size(serial::Bits8);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            settings.set_flow_control(serial::FlowNone);
            Ok(())
        }).unwrap();
        port.set_timeout(Duration::from_millis(1000)).unwrap();
        self.port = Option::from(port);
    }

    pub fn read(&mut self) -> io::Result<Vec<u8>> {
        let mut port = self.port.take().expect("Serial port is closed");

        let mut buffer: Vec<u8> = Vec::new();
        let _size = SerialStream::buffered_read(&mut buffer, &mut port);

        self.port = Option::from(port);
        Ok(buffer)
    }

    pub fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        let mut port = self.port.take().expect("Serial port is closed");

        let write_size = port.write(&data).expect("Serial write error");

        // Read echo response of written data from serial stream
        let mut buffer = vec![0 as u8; data.len() + 2]; // +2 is for CR LF (\r\n)
        let _check_size = port.read_exact(&mut buffer).expect("Serial read error");

        self.port = Option::from(port);
        Ok(write_size)
    }

    fn buffered_read<T: SerialPort>(buffer: &mut Vec<u8>, port: &mut T) -> usize {
        let buff_size: usize = 128;
        let mut total_size: usize = 0;
        let mut read_buff: Vec<u8> = vec![0; buff_size];

        loop {
            let read_size = match port.read(&mut read_buff) {
                Ok(n) => {
                    if n == 0 {
                        break
                    }
                    n
                },
                Err(_e) => break
            };
            buffer.extend(read_buff.split(|b| *b == 0).next().unwrap().to_vec());
            total_size += read_size;
            if read_size < buff_size {
                break;
            }
        }

        return total_size;
    }

    fn baud_to_enum(baud: usize) -> BaudRate {
        return match baud {
            110 => BaudRate::Baud110,
            300 => BaudRate::Baud300,
            600 => BaudRate::Baud600,
            1200 => BaudRate::Baud1200,
            2400 => BaudRate::Baud2400,
            4800 => BaudRate::Baud4800,
            9600 => BaudRate::Baud9600,
            19200 => BaudRate::Baud19200,
            38400 => BaudRate::Baud38400,
            57600 => BaudRate::Baud57600,
            115200 => BaudRate::Baud115200,
            n => BaudRate::BaudOther(n)
        };
    }
}