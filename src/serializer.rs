use crate::data::YAMLData;
use crate::error::YAMLParseError;

pub fn serialize(data: &YAMLData) -> Result<String, YAMLParseError> {
    match data {
        YAMLData::Scalar(value) => Ok(value.clone()),
        _ => Err(YAMLParseError::SerializationError),
    }
}