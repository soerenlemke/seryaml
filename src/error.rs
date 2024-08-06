#[derive(Debug)]
pub enum YAMLParseError {
    InvalidFormat,
    SerializationError,
    DeserializationError,
}