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
        println!("Executing send command");
        return Response::new(true, Vec::new());
    }
}