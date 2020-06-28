use std::error::Error;

use crate::command;

pub struct Help {
}

impl command::Base for Help {
    fn execute(&self) -> Result<(), Box<dyn Error>> {

        println!("");
        println!("Usage: watt <command> [options]");
        println!("");
        println!("Commands:");
        println!("");
        println!("  init      Initialize a new project");
        println!("  generate  Generate a new UI item");
        println!("  help      I'm not doing \"get help\"");
        println!("");

        Ok(())
    }
}
