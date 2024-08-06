use crate::data::YAMLData;
use crate::error::YAMLParseError;

pub fn parse(yaml: &str) -> Result<YAMLData, YAMLParseError> {
    Ok(YAMLData::Scalar(yaml.trim().to_string()))
}