use super::data_type::DataType;

mod number;
mod byte_string;
mod list;
mod dictionary;

pub fn parse(data:  &DataType) -> String {
    let result: String;
    match data {
        DataType::Number(num) => {
            result = number::parse(*num);
        },
        DataType::ByteString(string) => {
            result = byte_string::parse(&string);
        },
        DataType::List(list) => {
            result = list::parse(list);
        },
        DataType::Dictionary(dictionary) => {
            result = dictionary::parse(dictionary);
        },
        _ => {
            result = String::new();
        }
    }
    result
}