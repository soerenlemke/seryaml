#[cfg(test)]
mod tests {
    use seryaml::parser::parse;
    use seryaml::serializer::serialize;
    use seryaml::data::YAMLData;

    #[test]
    fn test_parse_scalar() {
        let yaml = "hello";
        let result = parse(yaml).unwrap();
        assert_eq!(result, YAMLData::Scalar("hello".to_string()));
    }

    #[test]
    fn test_serialize_scalar() {
        let data = YAMLData::Scalar("hello".to_string());
        let result = serialize(&data).unwrap();
        assert_eq!(result, "hello");
    }

}