use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct CommandParsingError {
    details: String
}

impl CommandParsingError {
    pub fn new(msg: &str) -> Box<CommandParsingError> {
        Box::new(CommandParsingError{details: msg.to_string()})
    }
}

impl fmt::Display for CommandParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for CommandParsingError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug)]
pub struct CommandExecutionError {
    details: String
}

impl CommandExecutionError {
    pub fn new(msg: &str) -> Box<CommandExecutionError> {
        Box::new(CommandExecutionError{details: msg.to_string()})
    }
}

impl fmt::Display for CommandExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for CommandExecutionError {
    fn description(&self) -> &str {
        &self.details
    }
}
