use std::iter::Peekable;
use std::collections::HashMap;
use super::super::data_type::DataType;

pub fn parse<T: Iterator<Item = char>> (iterable: &mut Peekable<T>) -> DataType {
    let mut result: HashMap<String, DataType> = HashMap::new();
    let mut closed = false;

    while let Some(c) = iterable.peek() {
        match c {
            'd' => {
                iterable.next();
            },
            'e' => {
                iterable.next();
                closed = true;
                break;
            },
            _ => {
                let mut key = String::from("");
                if let DataType::ByteString(k) = super::marshall_input(iterable) {
                    key.push_str(&k);
                }
                let value  = super::marshall_input(iterable);
                result.insert(key, value);
            },
        }
    }

    if closed {
        DataType::Dictionary(result)
    } else {
        DataType::BadCoded
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn parse_dictionary(dictionary: &String) -> DataType {
        let mut iterable = dictionary.as_str().chars().peekable();
        parse(&mut iterable)
    }

    #[test]
    fn should_return_dictionary_type() {
        // given
        let dictionary = String::from("d4:jojoi420ee");

        // when
        let decoded = parse_dictionary(&dictionary);

        // then
        match decoded {
            DataType::Dictionary(_) => (),
            _ => panic!("Data should be parsed as dictionary")
        }
    }

    #[test]
    fn should_return_given_dictionary_key() {
        // given
        let dictionary = String::from("d4:jojoi420ee");

        // when
        let decoded = parse_dictionary(&dictionary);

        // then
        match decoded {
            DataType::Dictionary(dictionary) => {
                if !dictionary.contains_key("jojo") {
                    panic!("dictionary should contain given key");
                }
            },
            _ => ()
        }
    }

    #[test]
    fn should_return_given_dictionary() {
        // given
        let dictionary = String::from("d4:jojoi420ee");

        // when
        let decoded = parse_dictionary(&dictionary);

        // then
        match decoded {
            DataType::Dictionary(dictionary) => {
                if let Some(value) = dictionary.get("jojo") {
                    if let DataType::Number(num) = value {
                        assert_eq!(*num, 420);
                    }
                }
            },
            _ => ()
        }
    }

    #[test]
    fn should_error_without_closing() {
        // given
        let dictionary = String::from("d4:jojoi420e");

        // when
        let decoded = parse_dictionary(&dictionary);

        // then
        match decoded {
            DataType::BadCoded => (),
            _ => panic!("Should error without closing"),
        }
    }

    #[test]
    fn should_error_on_bad_key() {
        // given
        let dictionary = String::from("di42ei420e");

        // when
        let decoded = parse_dictionary(&dictionary);

        // then
        match decoded {
            DataType::BadCoded => (),
            _ => panic!("Should error on bad key"),
        }
    }

    #[test]
    fn should_contain_another_dictionary() {
        // given
        let dictionary = String::from("d4:dictd3:key5:valueee");

        // when
        let decoded = parse_dictionary(&dictionary);

        // then
        match decoded {
            DataType::Dictionary(dictionary) => {
                if let Some(value) = dictionary.get("dict") {
                    if let DataType::Dictionary(dict) = value {
                        if let Some(v) = dict.get("key") {
                            if let DataType::ByteString(string) = v {
                                assert_eq!(*string, String::from("value"));
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }

    #[test]
    fn should_contain_another_list() {
        // given
        let dictionary = String::from("d4:listli420eee");

        // when
        let decoded = parse_dictionary(&dictionary);

        // then
        if let DataType::Dictionary(dictionary) = decoded {
            if let Some(list) = dictionary.get("list") {
                if let DataType::List(parsed_list) = list {
                    if let DataType::Number(number) = parsed_list[0] {
                        assert_eq!(number, 420);
                    }
                }
            }
        }
    }
}