use std::error::Error;

use crate::command;
use crate::error;

pub struct Generate {
}

impl command::Base for Generate {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        Err(error::CommandExecution::new("Not implemented yet."))
    }
}
