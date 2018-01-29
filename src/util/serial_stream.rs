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

    pub fn read(&mut self) -> io::Result<String> {
        let mut port = self.port.take().expect("Serial port is closed");

        let mut buffer = Vec::new();
        let _ = port.read(&mut buffer);

        self.port = Option::from(port);
        Ok(String::from("New message"))
    }

    pub fn write(&mut self) -> io::Result<usize> {
        let mut port = self.port.take().expect("Serial port is closed");

        let buffer = Vec::new();
        let size = port.write(&buffer[..]);

        self.port = Option::from(port);
        Ok(size.unwrap())
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