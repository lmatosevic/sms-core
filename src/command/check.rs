use util::serial_stream::SerialStream;
use super::model::Command;
use super::model::Response;

pub struct CheckConnection {}

impl CheckConnection {
    pub fn new() -> CheckConnection {
        CheckConnection {}
    }
}

impl Command for CheckConnection {
    fn execute(&self, serial_stream: &mut SerialStream) -> Response {
        let _size = serial_stream.write("AT\r".as_bytes()).unwrap();
        let response = serial_stream.read().unwrap();
        let _ = serial_stream.read().unwrap();
        return Response::new(true, response);
    }
}