mod number;
mod byte_string;
mod list;
mod dictionary;

use std::iter::Peekable;
use super::data_type::DataType;

pub fn parse(s: &String) -> DataType {
    let mut byte_string = s.as_str().chars().peekable();
    let result = marshall_input(&mut byte_string);
    result
}

fn marshall_input<T: Iterator<Item = char>> (iterator: &mut Peekable<T>) -> DataType {
    let result: DataType;
    if let Some(c) = iterator.peek() {
        match c {
            'i' => {
                result = number::parse(iterator);
            },
            '0'...'9' => {
                result = byte_string::parse(iterator);
            },
            'l' => {
                result = list::parse(iterator);
            },
            'd' => {
                result = dictionary::parse(iterator);
            },
            _ => result = DataType::BadCoded,
            
        }
    } else {
        result = DataType::BadCoded;
    }
    result
}