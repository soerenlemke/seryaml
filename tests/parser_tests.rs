use seryaml::data::YAMLData;
use seryaml::parser::parse_yaml;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sequence_of_scalars() {
        let yaml = "- Mark McGwire\n- Sammy Sosa\n- Ken Griffey";
        let result = parse_yaml(yaml).unwrap();
        let expected = YAMLData::Sequence(vec![
            YAMLData::Scalar("Mark McGwire".to_string()),
            YAMLData::Scalar("Sammy Sosa".to_string()),
            YAMLData::Scalar("Ken Griffey".to_string()),
        ]);
        assert_eq!(result, expected);
    }
}