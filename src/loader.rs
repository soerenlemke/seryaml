use std::fs::File;
use std::io;
use std::io::Read;

use crate::data::YAMLData;
use crate::error::YAMLParseError;
use crate::parser::parse_yaml;

pub fn read_file_to_string(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    Ok(contents)
}

pub fn load_yaml_file(filename: &str) -> Result<YAMLData, YAMLParseError> {
    let contents = read_file_to_string(filename).map_err(|_| YAMLParseError::InvalidFormat)?;
    
    parse_yaml(&contents)
}