mod integer;
mod byte_string;
mod list;

use std::iter::Peekable;
use super::data_type::DataType;

fn parse(s: &String) -> DataType {
    let mut byte_string = s.as_str().chars().peekable();
    let result = marshall_input(&mut byte_string);
    result
}

fn marshall_input<T: Iterator<Item = char>> (iterator: &mut Peekable<T>) -> DataType {
    let result: DataType;
    if let Some(c) = iterator.peek() {
        match c {
            'i' => {
                result = integer::parse(iterator);
            },
            '0'...'9' => {
                result = byte_string::parse(iterator);
            },
            'l' => {
                result = list::parse(iterator);
            },
            _ => result = DataType::BadCoded,
            
        }
    } else {
        result = DataType::BadCoded;
    }
    result
}