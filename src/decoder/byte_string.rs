use std::iter::Peekable;
use super::super::data_type::DataType;

pub fn parse<T: Iterator<Item = char>> (iterable: &mut Peekable<T>) -> DataType {
    let mut number = String::from("");
    let mut closed = false;
    let result: DataType;

    while let Some(c) = iterable.peek() {
        match c {
            '0'...'9' => number.push(*c),
            ':' => {
                iterable.next();
                closed = true;
                break;
            }
            _ => ()
        }
        iterable.next();
    }

    if closed {
        if let Ok(number) = number.parse::<i64>() {
            let mut byte_string = String::from("");
            for _ in 0..number {
                if let Some(c) = iterable.next() {
                    byte_string.push(c);
                }
            }
            result = DataType::ByteString(byte_string);
        } else {
            result = DataType::BadCoded;
        }
    } else {
        result = DataType::BadCoded;
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    fn parse_string(string: &String) -> DataType {
        let mut iterable = string.as_str().chars().peekable();
        parse(&mut iterable)
    }
    
    #[test]
    fn should_return_string_type() {
        // given
        let string = String::from("4:hamz");

        // when
        let decoded = parse_string(&string);

        // then
        match decoded {
            DataType::ByteString(_) => (),
            _ => panic!("Should not be this type")
        }
    }

    #[test]
    fn  should_error_without_colon() {
        // given
        let string = String::from("4hamz");

        // when
        let decoded = parse_string(&string);

        // then
        match decoded {
            DataType::BadCoded => (),
            _ => panic!("Should be treated as bad coded"),
        }
    }

    #[test]
    fn should_return_given_string() {
        // given
        let  string = String::from("4:hamz");

        // when
        let decoded = parse_string(&string);

        // then
        match decoded {
            DataType::ByteString(byte_string) => assert_eq!(byte_string, String::from("hamz")),
            _ => (),
        }
    }
}