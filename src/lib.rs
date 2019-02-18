pub mod data_type;
mod decoder;

pub fn decode(string: &String) -> data_type::DataType {
    let result = decoder::parse(string);
    result
}