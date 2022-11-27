use std::error::Error;
use std::fs;

pub fn read_file(filepath: &String) -> Result<String, Box<dyn Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}