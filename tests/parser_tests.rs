use seryaml::data::YAMLData;
use seryaml::parser::parse_yaml;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_parse_yaml_scalar() {
        let yaml = "hello";
        let result = parse_yaml(yaml).unwrap();
        assert_eq!(result, YAMLData::Scalar("hello".to_string()));
    }

    #[test]
    fn test_parse_yaml_sequence() {
        let yaml = "- item1\n- item2\n- item3";
        let expected = YAMLData::Sequence(vec![
            YAMLData::Scalar("item1".to_string()),
            YAMLData::Scalar("item2".to_string()),
            YAMLData::Scalar("item3".to_string())
        ]);
        let result = parse_yaml(yaml).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_yaml_mapping() {
        let yaml = "key1: value1\nkey2: value2";
        let mut expected_map = HashMap::new();
        expected_map.insert("key1".to_string(), YAMLData::Scalar("value1".to_string()));
        expected_map.insert("key2".to_string(), YAMLData::Scalar("value2".to_string()));

        let expected = YAMLData::Mapping(expected_map);
        let result = parse_yaml(yaml).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_yaml_nested_structure() {
        let yaml = "key1:\n  - item1\n  - item2\nkey2: value2";
        let mut expected_map = HashMap::new();
        expected_map.insert("key1".to_string(), YAMLData::Sequence(vec![
            YAMLData::Scalar("item1".to_string()),
            YAMLData::Scalar("item2".to_string())
        ]));
        expected_map.insert("key2".to_string(), YAMLData::Scalar("value2".to_string()));

        let expected = YAMLData::Mapping(expected_map);
        let result = parse_yaml(yaml).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_yaml_invalid() {
        let yaml = "invalid_yaml: [unbalanced";
        let result = parse_yaml(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_yaml_empty() {
        let yaml = "";
        let result = parse_yaml(yaml);
        assert!(result.is_err());
    }
}