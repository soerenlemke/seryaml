use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum YAMLData {
    Scalar(String),
    Sequence(Vec<YAMLData>),
    Mapping(HashMap<String, YAMLData>),
}