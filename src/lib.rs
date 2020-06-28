use std::env;
use std::error::Error;
use std::collections::VecDeque;

mod error;
mod command;

pub fn make_command(env_args: env::Args) -> Result<Box<dyn command::Base>, Box<dyn Error>> {

    let mut args: VecDeque<String> = VecDeque::new();

    for arg in env_args.skip(1) {
        args.push_back(arg);
    }

    if args.is_empty() {
        return Ok(Box::new(command::Help {}));
    }

    let command_text = args.pop_front().unwrap();

    match &command_text[..] {
        "/?"|"-?"|"/h"|"-h"|"--help"|"help" => return Ok(Box::new(command::Help {})),
        _ => {
            match &command_text[..2] {
                "in" => return Ok(Box::new(command::Init {})),
                "ge" => return Ok(Box::new(command::Generate {})),
                _ => return Err(error::CommandParsing::new(&format!("Unknown command: \"{}\"", command_text)))
            };
        }
    }
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