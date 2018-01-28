use super::model::Command;

pub struct CheckConnection {}

impl CheckConnection {
    fn new() -> CheckConnection {
        CheckConnection {}
    }
}

impl Command for CheckConnection {
    fn execute(&self) {
        println!("Executing check command");
    }
}