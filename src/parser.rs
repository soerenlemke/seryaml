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

    if lines[0].starts_with('-') {
        parse_sequence(lines)
    } else if lines[0].contains(':') {
        parse_mapping(lines)
    } else {
        Ok(YAMLData::Scalar(lines.join(" ").trim().to_string()))
    }
}

fn parse_sequence(lines: &[&str]) -> Result<YAMLData, YAMLParseError> {
    let mut items = Vec::new();

    for line in lines {
        if line.starts_with('-') {
            let item = line.trim_start_matches('-').trim();
            if item.is_empty() {
                return Err(YAMLParseError::InvalidFormat);
            }
            match parse_yaml(item) {
                Ok(parsed_item) => items.push(parsed_item),
                Err(_) => return Err(YAMLParseError::InvalidFormat),
            }
        }
    }

    Ok(YAMLData::Sequence(items))
}

fn parse_mapping(lines: &[&str]) -> Result<YAMLData, YAMLParseError> {
    let mut map = HashMap::new();
    let mut key: Option<String> = None;
    let mut nested_lines = Vec::new();
    let mut current_indent = None;

    for line in lines {
        if let Some((k, v)) = line.split_once(':') {
            if let Some(current_key) = key.take() {
                let value = parse_lines(&nested_lines)?;
                map.insert(current_key, value);
                nested_lines.clear();
            }

            current_indent = Some(line.chars().take_while(|c| c.is_whitespace()).count());

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
            if line.chars().take(indent).all(|c| c.is_whitespace()) {
                nested_lines.push(line.trim_start_matches(' '));
            } else {
                return Err(YAMLParseError::InvalidFormat);
            }
        } else {
            return Err(YAMLParseError::InvalidFormat);
        }
    }

    if let Some(current_key) = key {
        let value = parse_lines(&nested_lines)?;
        map.insert(current_key, value);
    }

    Ok(YAMLData::Mapping(map))
}

fn is_invalid_value(value: &str) -> bool {
    value.starts_with('[') && !value.ends_with(']')
}
