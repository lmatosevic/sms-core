use std::io::{Error, ErrorKind};
use util::serial_stream::SerialStream;
use command::model::Command;
use command::model::Response;
use command::check::CheckConnection;
use command::send::SendSMS;

pub struct Executor;

impl Executor {
    pub fn run(data: &mut Vec<u8>, serial_stream: &mut SerialStream) -> Response {
        let command = Executor::parse_command(data);
        let response = match command {
            Ok(cmd) => cmd.execute(serial_stream),
            Err(e) => Response::new(false, Vec::from(format!("ERR:{}", e.to_string())))
        };
        return response;
    }

    // TinySMS protocol commands parser
    fn parse_command(data: &mut Vec<u8>) -> Result<Box<dyn Command + 'static>, Error> {
        let mut groups = data.split(|b| *b == 0x00);
        let zero_cmd = vec![0x30 as u8];
        let command_code_ref = groups.next().unwrap_or(&zero_cmd);
        if command_code_ref.len() > 1 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid command code"));
        }

        let command_code = command_code_ref.first().unwrap_or(&zero_cmd[0]);
        return match *command_code as char {
            '1' => {
                if groups.next().is_some() {
                    return Err(Error::new(ErrorKind::InvalidData,
                                          "Invalid command check"));
                }
                Ok(Box::new(CheckConnection::new()))
            }
            '2' => {
                let dest = groups.next();
                if !dest.is_some() {
                    return Err(Error::new(ErrorKind::InvalidData,
                                          "Invalid number of arguments for command send"));
                }
                let msg = groups.next();
                if !msg.is_some() {
                    return Err(Error::new(ErrorKind::InvalidData,
                                          "Invalid number of arguments for command send"));
                }
                Ok(Box::new(SendSMS::new(
                    String::from_utf8(Vec::from(dest.unwrap())).unwrap(),
                    String::from_utf8(Vec::from(msg.unwrap())).unwrap(),
                )))
            }
            _ => Err(Error::new(ErrorKind::InvalidData, "Invalid command code"))
        };
    }
}