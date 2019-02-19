use std::collections::HashMap;
use super::super::data_type::DataType;
use super::byte_string;

pub fn parse(dictionary: &HashMap<String, DataType>) -> String {
    let mut result = String::new();
    result.push('l');
    for (key, val) in dictionary.iter() {
        result.push_str(&byte_string::parse(key));
        result.push_str(&super::parse(val));
    }
    result.push('e');
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_given_dictionary() {
        // given
        let mut dictionary = HashMap::new();
        dictionary.insert(String::from("number"), DataType::Number(420));

        // when
        let result = parse(&dictionary);

        // then
        assert_eq!(result, String::from("l6:numberi420ee"));
    }

    #[test]
    fn should_contain_another_dictionary() {
        // given
        let mut dictionary = HashMap::new();
        dictionary.insert(String::from("number"), DataType::Number(420));
        let mut outer = HashMap::new();
        outer.insert(String::from("dictionary"), DataType::Dictionary(dictionary));

        // when
        let result = parse(&outer);

        // then
        assert_eq!(result, String::from("l10:dictionaryl6:numberi420eee"));
    }
}