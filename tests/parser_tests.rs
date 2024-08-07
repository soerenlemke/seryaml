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

    #[test]
    fn test_mapping_scalars_to_sequences() {
        let yaml = "american:\n  - Boston Red Sox\n  - Detroit Tigers\n  - New York Yankees\nnational:\n  - New York Mets\n  - Chicago Cubs\n  - Atlanta Braves";
        let result = parse_yaml(yaml).unwrap();
        let american = vec![
            YAMLData::Scalar("Boston Red Sox".to_string()),
            YAMLData::Scalar("Detroit Tigers".to_string()),
            YAMLData::Scalar("New York Yankees".to_string()),
        ];
        let national = vec![
            YAMLData::Scalar("New York Mets".to_string()),
            YAMLData::Scalar("Chicago Cubs".to_string()),
            YAMLData::Scalar("Atlanta Braves".to_string()),
        ];
        let mut map = std::collections::HashMap::new();
        map.insert("american".to_string(), YAMLData::Sequence(american));
        map.insert("national".to_string(), YAMLData::Sequence(national));
        let expected = YAMLData::Mapping(map);
        assert_eq!(result, expected);
    }
}