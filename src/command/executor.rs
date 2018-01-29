use util::serial_stream::SerialStream;
use command::check::CheckConnection;
use command::send::SendSMS;

pub struct Executor {
    serial_stream: SerialStream,
}

impl Executor {
    pub fn new(serial_stream: SerialStream) -> Executor {
        Executor { serial_stream }
    }
}