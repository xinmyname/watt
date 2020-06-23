use std::error::Error;

pub struct Config {
    pub command: String,
}

impl Config {

    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let command = args[1].clone();

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