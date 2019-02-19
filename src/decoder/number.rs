use std::iter::Peekable;
use super::super::data_type::DataType;

pub fn parse<T: Iterator<Item = char>> (iterable: &mut Peekable<T>) -> DataType {
    let mut number = String::from("");
    let mut closed: bool = false;
    let mut data_type: DataType;

    while let Some(c) = iterable.peek() {
        match c {
            'i' => (),
            '0'...'9' | '-' => {
                number.push(*c);
            },
            'e' => {
                closed = true;
                iterable.next();
                break;
            },
            _ => break
        }
        iterable.next();
    }

    if let Ok(parsed_number) = number.parse::<i64>() {
        data_type = DataType::Number(parsed_number);
    } else {
        data_type = DataType::BadCoded;
    }

    if !closed {
        data_type = DataType::BadCoded;
    }

    data_type
}

#[cfg(test)]
mod test {
    use super::*;

    fn parse_number(number: &String) -> DataType {
        let mut iterable = number.as_str().chars().peekable();
        parse(&mut iterable)
    }

    #[test]
    fn should_return_number_type() {
        // given
        let number = String::from("i42e");

        // when
        let decoded = parse_number(&number);

        // then
        match decoded {
            DataType::Number(_) => (),
            _ => panic!("Data should be parsed as number"),
        }
    }

    #[test]
    fn should_return_given_number() {
        // given
        let number = String::from("i420e");

        // when
        let decoded = parse_number(&number);

        // then
        match decoded {
            DataType::Number(num) => assert_eq!(num, 420),
            _ => ()
        }
    }

    #[test]
    fn should_return_negative_number() {
        // given
        let number = String::from("i-42e");

        // when
        let decoded = parse_number(&number);

        // then
        match decoded {
            DataType::Number(num) => assert_eq!(-42, num),
            _ => ()
        }
    }

    #[test]
    fn should_error_bad_number() {
        // given
        let number = String::from("i4-2e");

        // when
        let decoded = parse_number(&number);

        // then
        match decoded {
            DataType::BadCoded => (),
            _ => panic!("Should not parsed")
        }
    }

    #[test]
    fn should_error_without_closing() {
        // given
        let number = String::from("i42");

        // when
        let decoded = parse_number(&number);

        // then
        match decoded {
            DataType::Number(_) => panic!("should not parsed"),
            _ => ()
        }
    }
}