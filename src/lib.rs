use std::env;
use std::error::Error;
use std::collections::VecDeque;

mod errors;
mod command;

struct InitCommand {
}

impl command::Base for InitCommand {

    fn execute(&self) -> Result<(), Box<dyn Error>> {
        println!("INIT!");
        Ok(())
    }
}

struct GenerateCommand {
}

impl command::Base for GenerateCommand {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        Err(errors::CommandExecutionError::new("Not implemented yet."))
    }
}

pub fn make_command(env_args: env::Args) -> Result<Box<dyn command::Base>, Box<dyn Error>> {

    let mut args: VecDeque<String> = VecDeque::new();

    for arg in env_args.skip(1) {
        args.push_back(arg);
    }

    if args.len() < 1 {
        return Err(errors::CommandParsingError::new("Not enough arguments"));
    }

    Ok(Box::new(InitCommand {} ))
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    
    let cmd = make_command(std::env::args())?;
    cmd.execute()?;

    Ok(())
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