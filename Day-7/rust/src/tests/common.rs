use std::fs;

pub fn reader(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(fs::read_to_string(path)?.parse()?)
}