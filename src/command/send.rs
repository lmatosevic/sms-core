use std::string::String;
use super::model::Command;

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
    fn execute(&self) {
        println!("Executing send command");
    }
}