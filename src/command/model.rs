pub struct Result {
    success: bool
}

impl Result {
    fn new(success: bool) -> Result {
        Result { success }
    }
}

pub trait Command {
    fn execute(&self) -> Result;
}
