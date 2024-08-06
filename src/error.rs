#[derive(Debug)]
pub enum YAMLParseError {
    EmptyInput,
    InvalidFormat,
    SerializationError,
    DeserializationError,
}

pub enum FileLoadError {
    FileNotFound,
    FileNotReadable,
}