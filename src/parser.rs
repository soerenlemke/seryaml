use crate::data::YAMLData;
use crate::error::YAMLParseError;

pub fn parse(yaml: &str) -> Result<YAMLData, YAMLParseError> {
    if yaml.trim().is_empty() { 
        return Err(YAMLParseError::EmptyInput);
    }
    
    Ok(YAMLData::Scalar(yaml.trim().to_string()))
}