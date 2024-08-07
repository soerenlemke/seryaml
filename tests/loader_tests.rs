#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use seryaml::data::YAMLData;
    use seryaml::loader::{load_yaml_file, read_file_to_string};

    #[test]
    fn test_read_file_to_string() {
        let filename = "test_read_file_to_string.yaml";
        std::fs::write(filename, "hello world").unwrap();

        let result = read_file_to_string(filename).unwrap();
        assert_eq!(result, "hello world");

        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_load_yaml_file_scalar() {
        let filename = "test_scalar.yaml";
        std::fs::write(filename, "hello").unwrap();

        let result = load_yaml_file(filename).unwrap();
        assert_eq!(result, YAMLData::Scalar("hello".to_string()));

        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_load_yaml_file_sequence() {
        let filename = "test_sequence.yaml";
        std::fs::write(filename, "- item1\n- item2\n- item3").unwrap();

        let expected = YAMLData::Sequence(vec![
            YAMLData::Scalar("item1".to_string()),
            YAMLData::Scalar("item2".to_string()),
            YAMLData::Scalar("item3".to_string()),
        ]);

        let result = load_yaml_file(filename).unwrap();
        assert_eq!(result, expected);

        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_load_yaml_file_mapping() {
        let filename = "test_mapping.yaml";
        std::fs::write(filename, "key1: value1\nkey2: value2").unwrap();

        let mut expected_map = HashMap::new();
        expected_map.insert("key1".to_string(), YAMLData::Scalar("value1".to_string()));
        expected_map.insert("key2".to_string(), YAMLData::Scalar("value2".to_string()));
        let expected = YAMLData::Mapping(expected_map);

        let result = load_yaml_file(filename).unwrap();
        assert_eq!(result, expected);

        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_load_yaml_file_empty() {
        let filename = "test_empty.yaml";
        std::fs::write(filename, "").unwrap();

        let result = load_yaml_file(filename);
        assert!(result.is_err());

        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_load_yaml_file_not_found() {
        let filename = "nonexistent.yaml";

        let result = load_yaml_file(filename);
        assert!(result.is_err());
    }

    #[test]
    fn test_load_yaml_file_fruits() {
        let filename = "fruits.yaml";
        std::fs::write(filename, "fruits:\n  - apple\n  - banana\n  - cherry").unwrap();

        let mut expected_map = HashMap::new();
        let mut fruits = Vec::new();
        fruits.push(YAMLData::Scalar("apple".to_string()));
        fruits.push(YAMLData::Scalar("banana".to_string()));
        fruits.push(YAMLData::Scalar("cherry".to_string()));
        expected_map.insert("fruits".to_string(), YAMLData::Sequence(fruits));
        let expected = YAMLData::Mapping(expected_map);

        let result = load_yaml_file(filename).unwrap();
        assert_eq!(result, expected);

        std::fs::remove_file(filename).unwrap();
    }
}