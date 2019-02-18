use std::iter::Peekable;
use super::super::data_type::DataType;

pub fn parse<T: Iterator<Item = char>> (iterable: &mut Peekable<T>) -> DataType {
    let mut list: Vec<DataType> = Vec::new();
    let mut start = false;
    let mut closed = false;
    let result: DataType;
    while let Some(c) = iterable.peek() {
        match c {
            'l' => {
                if start {
                    list.push(super::marshall_input(iterable));
                }
                start = true;
                iterable.next();
            },
            'e' => {
                iterable.next();
                closed = true;
                break;
            },
            _ => {
                list.push(super::marshall_input(iterable));
            },
        }
    }

    if closed {
        result = DataType::List(list);
    } else {
        result = DataType::BadCoded;
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    fn parse_list(list: &String) -> DataType {
        let mut iterable = list.as_str().chars().peekable();
        parse(&mut iterable)
    }

    #[test]
    fn should_return_list_type() {
        // given
        let list = String::from("l4:hamzi420ee");

        // when
        let decoded = parse_list(&list);

        // then
        match decoded {
            DataType::List(_) => (),
            _ => panic!("Data should be parsed as list")
        }
    }

    #[test]
    fn should_return_given_list() {
        // given
        let list = String::from("l4:hamzi420ee");
        
        // when
        let decoded = parse_list(&list);

        // then
        match decoded {
            DataType::List(list) => {
                let mut equal = true;
                for i in 0..list.len() {
                    match &list[i] {
                        DataType::Number(integer) => {
                            equal = equal && (*integer == 420);
                        },
                        DataType::ByteString(string) => {
                            equal = equal && (*string == String::from("hamz"));
                        }
                        _ => ()
                    }
                }
                assert!(equal);
            },
            _ => (),
        }
    }

    #[test]
    fn should_error_without_closing() {
        // given
        let list = String::from("l4:hamzi420e");

        // when
        let decoded = parse_list(&list);
        
        // then
        match decoded {
            DataType::BadCoded => (),
            _ => panic!("Should not treated as valid")
        }
    }

    #[test]
    fn should_return_list_of_list() {
        // given
        let list = String::from("ll4:hamzei420e");

        // when
        let decoded = parse_list(&list);

        // then
        match decoded {
            DataType::List(list) => {
                match &list[0] {
                    DataType::List(_) => (),
                    _ => panic!("Should be parsed as list of list"),
                };
            },
            _ => ()
        }
    }

    #[test]
    fn should_return_list_of_dictionary() {
        // given
        let list = String::from("ld3:key5:valueee");

        // when
        let decoded = parse_list(&list);

        // then
        if let DataType::List(list) = decoded {
            if let DataType::Dictionary(dict) = &list[0] {
                if let Some(value) = dict.get("key") {
                    if let DataType::ByteString(string) = value {
                        assert_eq!(*string, String::from("value"));
                    }
                }
            }
        }
    }
}