use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum YAMLData {
    Scalar(String),
    Sequence(Vec<YAMLData>),
    Mapping(HashMap<String, YAMLData>),
}