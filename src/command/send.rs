use std::string::String;
use util::serial_stream::SerialStream;
use super::model::Command;
use super::model::Response;

pub struct SendSMS {
    destination: String,
    message: String,
}

impl SendSMS {
    pub fn new(destination: String, message: String) -> SendSMS {
        SendSMS { destination, message }
    }
}

impl Command for SendSMS {
    fn execute(&self, serial_stream: &mut SerialStream) -> Response {
        let _size = serial_stream.write("AT + CMGF = 1\r".as_bytes()).unwrap();
        let _response = serial_stream.read().unwrap();
        let _size = serial_stream.write(format!("AT + CMGS = \"{}\"\r", self.destination).as_bytes()).unwrap();
        let _response = serial_stream.read().unwrap();
        let _size = serial_stream.write(format!("{}\r", self.message).as_bytes()).unwrap();
        let _response = serial_stream.read().unwrap();
        let _size = serial_stream.write(&[0x1a]).unwrap();
        let _response = serial_stream.read().unwrap();
        return Response::new(true, Vec::from("OK\n".as_bytes()));
    }
}