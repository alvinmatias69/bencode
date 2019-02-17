use std::collections::HashMap;

pub enum DataType {
    Number(i64),
    ByteString(String),
    List(Vec<DataType>),
    Dictionary(HashMap<String, DataType>),

    BadCoded,

    // for mocking test only
    Undefined,
}