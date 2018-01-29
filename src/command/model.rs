use util::serial_stream::SerialStream;

pub struct Result {
    success: bool
}

impl Result {
    pub fn new(success: bool) -> Result {
        Result { success }
    }
}

pub trait Command {
    fn execute(&self, serial_stream: SerialStream) -> Result;
}
