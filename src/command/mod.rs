use std::error::Error;

pub trait Base {
    fn execute(&self) -> Result<(), Box<dyn Error>>;
}
