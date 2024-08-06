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

    #[test]
    fn test_parse_yaml_mixed_types() {
        let yaml = "key1: value1\nkey2:\n  - item1\n  - item2\nkey3: value3";
        let mut expected_map = HashMap::new();
        expected_map.insert("key1".to_string(), YAMLData::Scalar("value1".to_string()));
        expected_map.insert("key3".to_string(), YAMLData::Scalar("value3".to_string()));
        expected_map.insert("key2".to_string(), YAMLData::Sequence(vec![
            YAMLData::Scalar("item1".to_string()),
            YAMLData::Scalar("item2".to_string())
        ]));

        let expected = YAMLData::Mapping(expected_map);
        let result = parse_yaml(yaml).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_yaml_deeply_nested() {
        let yaml = "key1:\n  key2:\n    key3: value3";
        let mut expected_map_level2 = HashMap::new();
        expected_map_level2.insert("key3".to_string(), YAMLData::Scalar("value3".to_string()));

        let mut expected_map_level1 = HashMap::new();
        expected_map_level1.insert("key2".to_string(), YAMLData::Mapping(expected_map_level2));

        let mut expected_map = HashMap::new();
        expected_map.insert("key1".to_string(), YAMLData::Mapping(expected_map_level1));

        let expected = YAMLData::Mapping(expected_map);
        let result = parse_yaml(yaml).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_yaml_edge_cases() {
        let yaml = "key1: \nkey2:   value2";
        let mut expected_map = HashMap::new();
        expected_map.insert("key1".to_string(), YAMLData::Scalar("".to_string()));
        expected_map.insert("key2".to_string(), YAMLData::Scalar("value2".to_string()));

        let expected = YAMLData::Mapping(expected_map);
        let result = parse_yaml(yaml).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_yaml_multiline_scalar() {
        let yaml = "key1: |\n  line1\n  line2\nkey2: value2";
        let mut expected_map = HashMap::new();
        expected_map.insert("key1".to_string(), YAMLData::Scalar("line1\nline2".to_string()));
        expected_map.insert("key2".to_string(), YAMLData::Scalar("value2".to_string()));

        let expected = YAMLData::Mapping(expected_map);
        let result = parse_yaml(yaml).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_yaml_complex_structure() {
        let yaml = "key1:\n  - item1\n  - item2:\n      - subitem1\n      - subitem2\nkey3: value3";
        let sub_sequence = vec![
            YAMLData::Scalar("subitem1".to_string()),
            YAMLData::Scalar("subitem2".to_string()),
        ];

        let sequence = vec![
            YAMLData::Scalar("item1".to_string()),
            YAMLData::Mapping(HashMap::from([("item2".to_string(), YAMLData::Sequence(sub_sequence))]))
        ];

        let mut expected_map = HashMap::new();
        expected_map.insert("key1".to_string(), YAMLData::Sequence(sequence));
        expected_map.insert("key3".to_string(), YAMLData::Scalar("value3".to_string()));

        let expected = YAMLData::Mapping(expected_map);
        let result = parse_yaml(yaml).unwrap();
        assert_eq!(result, expected);
    }
}