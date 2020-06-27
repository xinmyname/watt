use std::env;
use std::error::Error;
use std::fmt;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct CommandParsingError {
    details: String
}

impl CommandParsingError {
    fn new(msg: &str) -> Box<CommandParsingError> {
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
    fn new(msg: &str) -> Box<CommandExecutionError> {
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


pub trait Command {
    fn execute(&self) -> Result<(), Box<dyn Error>>;
}

struct InitCommand {
}

impl Command for InitCommand {

    fn execute(&self) -> Result<(), Box<dyn Error>> {
        println!("INIT!");
        Ok(())
    }
}

struct GenerateCommand {
}

impl Command for GenerateCommand {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        Err(CommandExecutionError::new("Not implemented yet."))
    }
}

pub fn make_command(env_args: env::Args) -> Result<Box<dyn Command>, Box<dyn Error>> {

    let mut args: VecDeque<String> = VecDeque::new();

    for arg in env_args.skip(1) {
        args.push_back(arg);
    }

    if args.len() < 1 {
        return Err(CommandParsingError::new("not enough arguments"));
    }

    Ok(Box::new(InitCommand {} ))
}

pub fn run() {

    match make_command(std::env::args()) {

        Ok(cmd) => { 
            cmd.execute().unwrap_or_else(|err| {
                println!("ERROR: {}", err)
            });
        },        
        Err(err) => println!("ERROR: {}", err)
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn good_test() {
        assert_eq!(true, true);
    }

    #[test]
    fn bad_test() {
        assert_eq!(true, false);
    }
}