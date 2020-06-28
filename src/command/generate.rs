use std::error::Error;

use crate::command;
use crate::errors;

pub struct Generate {
}

impl command::Base for Generate {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        Err(errors::CommandExecutionError::new("Not implemented yet."))
    }
}
