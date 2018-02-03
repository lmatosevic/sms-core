use util::serial_stream::SerialStream;
use command::model::Command;
use command::model::Response;
use command::check::CheckConnection;
use command::send::SendSMS;

pub struct Executor;

impl Executor {
    pub fn run(data: &mut Vec<u8>, serial_stream: &mut SerialStream) -> Response {
        serial_stream.open();
        let command = Executor::parse_command(data);
        let response = match command {
            Some(cmd) => cmd.execute(serial_stream),
            None => Response::new(false, Vec::from("Invalid command"))
        };
        return response;
    }

    // TinySMS protocol commands
    fn parse_command(data: &mut Vec<u8>) -> Option<Box<Command + 'static>> {
        let command_code = data.first().unwrap(); // Check taht second value must be '\0'
        return match *command_code as char {
            '1' => Some(Box::new(CheckConnection::new())),
            '2' => Some(Box::new(SendSMS::new(
                String::from("+3859817585"),
                String::from("Hello world!")))),
            _ => None
        };
    }
}