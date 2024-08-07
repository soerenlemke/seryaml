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

    for line in lines {
        if line.starts_with('-') {
            results.push(parse_sequence(line)?);
        } else if line.contains(':') {
            results.push(parse_mapping(&[line])?);
        } else {
            results.push(YAMLData::Scalar(line.trim().to_string()));
        }
    }

    Ok(YAMLData::Sequence(results))
}

fn parse_sequence(line: &str) -> Result<YAMLData, YAMLParseError> {
    let item = line.trim_start_matches('-').trim();
    if item.is_empty() {
        return Err(YAMLParseError::InvalidFormat);
    }
    Ok(YAMLData::Scalar(item.to_string()))
}

fn parse_mapping(lines: &[&str]) -> Result<YAMLData, YAMLParseError> {
    let mut map = HashMap::new();
    let mut key: Option<String> = None;
    let mut nested_lines = Vec::new();
    let mut current_indent = None;

    for line in lines {
        if let Some((k, v)) = line.split_once(':') {
            // If there is a current key, finalize it
            if let Some(current_key) = key.take() {
                let value = parse_lines(&nested_lines)?;
                map.insert(current_key, value);
                nested_lines.clear();
            }

            // Determine the current indentation level
            current_indent = Some(line.chars().take_while(|c| c.is_whitespace()).count());

            // Set the new key
            key = Some(k.trim().to_string());

            let value = v.trim();
            if !value.is_empty() {
                if is_invalid_value(value) {
                    return Err(YAMLParseError::InvalidFormat);
                }
                map.insert(k.trim().to_string(), parse_yaml(value)?);
                key = None;
                current_indent = None;
            }
        } else if let Some(indent) = current_indent {
            // Check if the line is part of the current nested structure
            if line.chars().take(indent).all(|c| c.is_whitespace()) {
                nested_lines.push(line.trim_start_matches(' '));
            } else {
                if let Some(current_key) = key.take() {
                    let value = parse_lines(&nested_lines)?;
                    map.insert(current_key, value);
                    nested_lines.clear();
                }
                current_indent = None;
                key = None;
            }
        } else {
            return Err(YAMLParseError::InvalidFormat);
        }
    }

    // Finalize any remaining key
    if let Some(current_key) = key {
        let value = parse_lines(&nested_lines)?;
        map.insert(current_key, value);
    }

    Ok(YAMLData::Mapping(map))
}

fn is_invalid_value(value: &str) -> bool {
    // Simple check for invalid value
    value.starts_with('[') && !value.ends_with(']')
}
