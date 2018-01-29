use util::serial_stream::SerialStream;
use super::model::Command;
use super::model::Result;

pub struct CheckConnection {}

impl CheckConnection {
    fn new() -> CheckConnection {
        CheckConnection {}
    }
}

impl Command for CheckConnection {
    fn execute(&self, serial_stream: SerialStream) -> Result {
        println!("Executing check command");
        return Result::new(true);
    }
}