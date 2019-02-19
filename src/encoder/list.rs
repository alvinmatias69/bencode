use super::super::data_type::DataType;

pub fn parse(list: &Vec<DataType>) -> String {
    let mut result = String::new();
    result.push('l');
    for item in list.into_iter() {
        result.push_str(&super::parse(item));
    }
    result.push('e');
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_given_list() {
        // given
        let list: Vec<DataType> = vec![DataType::Number(420), DataType::ByteString(String::from("jotaro"))];

        // when
        let result = parse(&list);

        // then
        assert_eq!(result, "li420e6:jotaroe");
    }

    #[test]
    fn should_return_list_of_list() {
        // given
        let list: Vec<DataType> = vec![DataType::Number(420), DataType::ByteString(String::from("jotaro"))];
        let outer: Vec<DataType> = vec![DataType::List(list)];

        // when
        let result = parse(&outer);

        // then
        assert_eq!(result, "lli420e6:jotaroee");
    }
}