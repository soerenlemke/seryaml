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
    let mut mapping_lines = Vec::new();

    for line in lines {
        if line.starts_with('-') {
            if !mapping_lines.is_empty() {
                results.push(parse_mapping(&mapping_lines.iter().copied().collect::<Vec<&str>>().join("\n"))?);
                mapping_lines.clear();
            }
            results.push(parse_sequence(line)?);
        } else if line.contains(':') {
            mapping_lines.push(line);
        } else {
            if !mapping_lines.is_empty() {
                results.push(parse_mapping(&mapping_lines.join("\n"))?);
                mapping_lines.clear();
            }
            results.push(YAMLData::Scalar(line.trim().to_string()));
        }
    }

    if !mapping_lines.is_empty() {
        results.push(parse_mapping(&mapping_lines.iter().copied().collect::<Vec<&str>>().join("\n"))?);
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

fn parse_mapping(lines: &str) -> Result<YAMLData, YAMLParseError> {
    let mut map = HashMap::new();
    for line in lines.lines() {
        if let Some((k, v)) = line.split_once(':') {
            let key = k.trim().to_string();
            let value = v.trim().to_string();
            if key.is_empty() || value.is_empty() {
                return Err(YAMLParseError::InvalidFormat);
            }
            map.insert(key, YAMLData::Scalar(value));
        } else {
            return Err(YAMLParseError::InvalidFormat);
        }
    }
    Ok(YAMLData::Mapping(map))
}
