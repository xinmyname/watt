use std::env;
use std::error::Error;
use std::fmt;
use std::collections::VecDeque;

pub enum Command {
    Unknown,
    Init,
    Generate
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::Unknown => write!(f, "Unknown"),
            Command::Init => write!(f, "Init"),
            Command::Generate => write!(f, "Generate")
        }
    }
}

pub struct Config {
    pub command: Command,
}

impl Config {


    pub fn new(env_args: env::Args) -> Result<Config, &'static str> {

        let mut args: VecDeque<String> = VecDeque::new();

        for arg in env_args.skip(1) {
            args.push_back(arg);
        }

        if args.len() < 1 {
            return Err("not enough arguments");
        }

        let command_arg = args.pop_front().unwrap();

        println!("command arg: {}", command_arg);

        let command = Command::Unknown;

        Ok(Config {
            command
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    println!("Command: {}", config.command);

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