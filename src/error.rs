#[derive(Debug)]
pub enum YAMLParseError {
    EmptyInput,
    InvalidFormat,
    SerializationError,
    DeserializationError,
}