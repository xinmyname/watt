use std::error::Error;

pub trait Base {
    fn execute(&self) -> Result<(), Box<dyn Error>>;
}

mod init;
mod generate;
mod help;

pub use {init::Init, generate::Generate, help::Help };

