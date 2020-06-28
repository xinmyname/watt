use std::env;
use std::error::Error;
use std::collections::VecDeque;

mod errors;
mod command;



pub fn make_command(env_args: env::Args) -> Result<Box<dyn command::Base>, Box<dyn Error>> {

    let mut args: VecDeque<String> = VecDeque::new();

    for arg in env_args.skip(1) {
        args.push_back(arg);
    }

    if args.len() < 1 {
        return Err(errors::CommandParsingError::new("Not enough arguments"));
    }

    Ok(Box::new(command::Init {} ))
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