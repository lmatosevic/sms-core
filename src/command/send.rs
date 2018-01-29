use std::string::String;
use util::serial_stream::SerialStream;
use super::model::Command;
use super::model::Result;

pub struct SendSMS {
    destination: String,
    message: String,
}

impl SendSMS {
    fn new(destination: String, message: String) -> SendSMS {
        SendSMS { destination, message }
    }
}

impl Command for SendSMS {
    fn execute(&self, serial_stream: SerialStream) -> Result {
        println!("Executing send command");
        return Result::new(true);
    }
}