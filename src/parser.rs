use std::collections::HashMap;

use crate::data::YAMLData;
use crate::error::YAMLParseError;

pub fn parse_yaml(yaml: &str) -> Result<YAMLData, YAMLParseError> {
    if yaml.trim().is_empty() {
        return Err(YAMLParseError::InvalidFormat);
    }

    let lines: Vec<&str> = yaml.lines().collect();
    parse_lines(&lines)
}

fn parse_lines(lines: &[&str]) -> Result<YAMLData, YAMLParseError> {
    if lines.is_empty() {
        return Err(YAMLParseError::InvalidFormat);
    }

    let mut results = Vec::new();
    let mut current_mapping = HashMap::new();
    let mut current_key = None;
    let mut current_sequence = Vec::new();
    let mut current_indent = 0;
    let mut in_sequence = false;

    for line in lines {
        let trimmed_line = line.trim();
        let line_indent = line.chars().take_while(|c| c.is_whitespace()).count();

        if trimmed_line.starts_with('-') {
            if let Some(_key) = &current_key {
                current_sequence.push(parse_sequence(trimmed_line)?);
                in_sequence = true;
            } else {
                if !current_mapping.is_empty() {
                    results.push(YAMLData::Mapping(current_mapping));
                    current_mapping = HashMap::new();
                }
                results.push(parse_sequence(trimmed_line)?);
            }
        } else if trimmed_line.contains(':') {
            if in_sequence && line_indent <= current_indent {
                if let Some(key) = current_key.take() {
                    current_mapping.insert(key, YAMLData::Sequence(current_sequence));
                    current_sequence = Vec::new();
                }
                in_sequence = false;
            }
            let (key, value) = trimmed_line.split_once(':').unwrap();
            current_key = Some(key.trim().to_string());
            current_indent = line_indent;
            if !value.trim().is_empty() {
                current_mapping.insert(current_key.clone().unwrap(), YAMLData::Scalar(value.trim().to_string()));
                current_key = None;
            }
        } else if trimmed_line.is_empty() {
            continue;
        } else {
            if let Some(key) = current_key.take() {
                if in_sequence {
                    current_mapping.insert(key, YAMLData::Sequence(current_sequence));
                    current_sequence = Vec::new();
                }
            }
            if !current_mapping.is_empty() {
                results.push(YAMLData::Mapping(current_mapping));
                current_mapping = HashMap::new();
            }
            results.push(YAMLData::Scalar(trimmed_line.to_string()));
        }
    }

    if let Some(key) = current_key {
        if !current_sequence.is_empty() {
            current_mapping.insert(key, YAMLData::Sequence(current_sequence));
        }
    }

    if !current_mapping.is_empty() {
        results.push(YAMLData::Mapping(current_mapping));
    }

    if results.len() == 1 {
        Ok(results.remove(0))
    } else {
        Ok(YAMLData::Sequence(results))
    }
}

fn parse_sequence(line: &str) -> Result<YAMLData, YAMLParseError> {
    let item = line.trim_start_matches('-').trim();
    if item.is_empty() {
        return Err(YAMLParseError::InvalidFormat);
    }
    Ok(YAMLData::Scalar(item.to_string()))
}