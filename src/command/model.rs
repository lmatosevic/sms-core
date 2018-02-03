use util::serial_stream::SerialStream;

pub struct Response {
    pub success: bool,
    pub data: Vec<u8>,
}

impl Response {
    pub fn new(success: bool, data: Vec<u8>) -> Response {
        Response { success, data }
    }
}

pub trait Command {
    fn execute(&self, serial_stream: &mut SerialStream) -> Response;
}
