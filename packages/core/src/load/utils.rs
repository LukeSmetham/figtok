use std::fs;
use std::path::Path;

// Define a custom error type for the read_file() function.
#[derive(Debug)]
pub enum ReadFileError {
    InvalidFilepath,
    IoError(std::io::Error),
}

impl From<std::io::Error> for ReadFileError {
    fn from(error: std::io::Error) -> Self {
        ReadFileError::IoError(error)
    }
}

pub fn read_file(filepath: &String) -> Result<String, ReadFileError> {
    let path = Path::new(filepath);
    if !path.is_file() {
        return Err(ReadFileError::InvalidFilepath);
    }

    let data = match fs::read_to_string(filepath) {
        Ok(data) => data,
        Err(error) => return Err(ReadFileError::from(error)),
    };

    Ok(data)
}
