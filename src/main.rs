fn main() {
    
    match watt::run() {
        Ok(_) => {},
        Err(err) => println!("ERROR: {}", err)
    }
}

