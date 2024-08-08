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
    let mut results = Vec::new();
    let mut current_mapping = HashMap::new();
    let mut current_sequence = Vec::new();
    let mut current_key = None;
    let mut in_sequence = false;
    let mut sequence_indent = None;

    for line in lines {
        let trimmed_line = line.trim();
        let indent_level = line.chars().take_while(|c| c.is_whitespace()).count();

        if is_sequence(trimmed_line) {
            handle_sequence(
                &mut results,
                &mut current_mapping,
                &mut current_sequence,
                &mut current_key,
                &mut in_sequence,
                &mut sequence_indent,
                trimmed_line,
                indent_level,
            )?;
        } else if is_mapping(trimmed_line) {
            handle_mapping(
                &mut results,
                &mut current_mapping,
                &mut current_sequence,
                &mut current_key,
                &mut in_sequence,
                &mut sequence_indent,
                trimmed_line,
                indent_level,
            )?;
        } else if trimmed_line.is_empty() {
            continue;
        } else {
            handle_scalar(&mut current_mapping, &mut current_key, trimmed_line);
        }
    }

    finalize_results(&mut results, &mut current_mapping, &mut current_sequence, in_sequence);

    if results.len() == 1 {
        Ok(results.remove(0))
    } else {
        Ok(YAMLData::Sequence(results))
    }
}

fn handle_sequence(
    results: &mut Vec<YAMLData>,
    current_mapping: &mut HashMap<String, YAMLData>,
    current_sequence: &mut Vec<YAMLData>,
    current_key: &mut Option<String>,
    in_sequence: &mut bool,
    sequence_indent: &mut Option<usize>,
    trimmed_line: &str,
    indent_level: usize,
) -> Result<(), YAMLParseError> {
    let item = parse_sequence_item(trimmed_line)?;
    if *in_sequence {
        current_sequence.push(item);
    } else {
        finalize_results(results, current_mapping, current_sequence, *in_sequence);
        *in_sequence = true;
        *sequence_indent = Some(indent_level);
        current_sequence.push(item);
    }
    Ok(())
}

fn handle_mapping(
    results: &mut Vec<YAMLData>,
    current_mapping: &mut HashMap<String, YAMLData>,
    current_sequence: &mut Vec<YAMLData>,
    current_key: &mut Option<String>,
    in_sequence: &mut bool,
    sequence_indent: &mut Option<usize>,
    trimmed_line: &str,
    indent_level: usize,
) -> Result<(), YAMLParseError> {
    let (key, value) = parse_mapping_item(trimmed_line)?;

    if *in_sequence && indent_level > sequence_indent.unwrap_or(0) {
        if let Some(ref current_key) = current_key {
            let nested_mapping = current_mapping.entry(current_key.clone()).or_insert_with(|| YAMLData::Mapping(HashMap::new()));
            if let YAMLData::Mapping(ref mut nested_mapping) = nested_mapping {
                nested_mapping.insert(key, YAMLData::Scalar(value));
            }
        } else {
            current_mapping.insert(key, YAMLData::Scalar(value));
        }
    } else {
        finalize_results(results, current_mapping, current_sequence, *in_sequence);
        *in_sequence = false;
        *sequence_indent = None;
        current_mapping.insert(key.clone(), YAMLData::Scalar(value));
        *current_key = Some(key);
    }
    Ok(())
}

fn handle_scalar(
    current_mapping: &mut HashMap<String, YAMLData>,
    current_key: &mut Option<String>,
    trimmed_line: &str,
) {
    if let Some(key) = current_key.take() {
        current_mapping.insert(key, YAMLData::Scalar(trimmed_line.to_string()));
    }
}

fn finalize_results(
    results: &mut Vec<YAMLData>,
    current_mapping: &mut HashMap<String, YAMLData>,
    current_sequence: &mut Vec<YAMLData>,
    in_sequence: bool,
) {
    if in_sequence {
        if !current_mapping.is_empty() {
            current_sequence.push(YAMLData::Mapping(std::mem::take(current_mapping)));
        }
        results.push(YAMLData::Sequence(std::mem::take(current_sequence)));
    } else if !current_mapping.is_empty() {
        results.push(YAMLData::Mapping(std::mem::take(current_mapping)));
    }
}

fn is_sequence(line: &str) -> bool {
    line.starts_with('-')
}

fn is_mapping(line: &str) -> bool {
    line.contains(':')
}

fn parse_sequence_item(line: &str) -> Result<YAMLData, YAMLParseError> {
    let item = line.trim_start_matches('-').trim();
    if item.is_empty() {
        return Err(YAMLParseError::InvalidFormat);
    }
    Ok(YAMLData::Scalar(item.to_string()))
}

fn parse_mapping_item(line: &str) -> Result<(String, String), YAMLParseError> {
    let (key, value) = line.split_once(':').unwrap();
    Ok((key.trim().to_string(), value.trim().to_string()))
}