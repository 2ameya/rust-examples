use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("नमस्ते, world!");
    Ok(())
}