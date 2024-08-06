use std::fs::File;
use std::{fs, io};
use std::io::Read;
use FileLoadError::FileNotFound;
use crate::data::YAMLData;
use crate::error::{FileLoadError, YAMLParseError};
use crate::parser::parse;

pub fn read_file_to_string(filename: &str) -> Result<String, io::Error> {
    if fs::metadata(filename).is_err() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    }
    
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    Ok(contents)
}

pub fn load_yaml_file(filename: &str) -> Result<YAMLData, YAMLParseError> {
    let contents = read_file_to_string(filename).map_err(|_| YAMLParseError::InvalidFormat)?;
    
    parse(&contents)
}