pub mod data_type;
mod decoder;
mod encoder;

pub fn decode(string: &String) -> data_type::DataType {
    let result = decoder::parse(string);
    result
}

pub fn encode(data: &data_type::DataType) -> String {
    let  result = encoder::parse(data);
    result
}