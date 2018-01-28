extern crate serial;

use self::serial::BaudRate;
use self::serial::prelude::*;
use std::time::Duration;

pub struct SerialStream {
    device: String,
    baud: usize,
}

impl SerialStream {
    pub fn new(device: String, baud: usize) -> SerialStream {
        SerialStream { device, baud }
    }

    pub fn open(&self) {
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