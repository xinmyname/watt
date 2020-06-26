use std::process;
use watt::Config;

fn main() {
    

    let config = Config::new(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = watt::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}