use std::error::Error;

use crate::command;

pub struct Init {
}

impl command::Base for Init {

    fn execute(&self) -> Result<(), Box<dyn Error>> {
        println!("INIT!");
        Ok(())
    }
}