use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct CommandParsing {
    details: String
}

impl CommandParsing {
    pub fn new(msg: &str) -> Box<CommandParsing> {
        Box::new(CommandParsing{details: msg.to_string()})
    }
}

impl fmt::Display for CommandParsing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for CommandParsing {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug)]
pub struct CommandExecution {
    details: String
}

impl CommandExecution {
    pub fn new(msg: &str) -> Box<CommandExecution> {
        Box::new(CommandExecution{details: msg.to_string()})
    }
}

impl fmt::Display for CommandExecution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for CommandExecution {
    fn description(&self) -> &str {
        &self.details
    }
}
