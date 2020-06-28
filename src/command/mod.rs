use std::error::Error;

pub trait Base {
    fn execute(&self) -> Result<(), Box<dyn Error>>;
}

mod init;
mod generate;

pub use {init::Init, generate::Generate };

