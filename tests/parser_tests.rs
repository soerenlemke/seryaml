use seryaml::data::YAMLData;
use seryaml::parser::parse_yaml;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_of_scalars() {
        let yaml = "- Mark McGwire\n- Sammy Sosa\n- Ken Griffey";
        let result = parse_yaml(yaml).unwrap();
        let expected = YAMLData::Sequence(vec![
            YAMLData::Scalar("Mark McGwire".to_string()),
            YAMLData::Scalar("Sammy Sosa".to_string()),
            YAMLData::Scalar("Ken Griffey".to_string()),
        ]);
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_mapping_of_scalars() {
        let yaml = "hr: 65\navg: 0.278\nrbi: 147";
        let result = parse_yaml(yaml).unwrap();
        let mut map = std::collections::HashMap::new();
        map.insert("hr".to_string(), YAMLData::Scalar("65".to_string()));
        map.insert("avg".to_string(), YAMLData::Scalar("0.278".to_string()));
        map.insert("rbi".to_string(), YAMLData::Scalar("147".to_string()));
        let expected = YAMLData::Mapping(map);
        assert_eq!(result, expected);
    }
}