use std::fs;
use std::net::SocketAddr;
use std::error::Error;

pub fn file_to_string(file_name: &str) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(file_name)?.parse()?)
}

