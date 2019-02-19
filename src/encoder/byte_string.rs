pub fn parse(string: &String) -> String {
    let count = string.len();
    let mut result = String::new();
    result.push_str(&count.to_string());
    result.push(':');
    result.push_str(string);
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_given_string() {
        // given
        let string = String::from("jotaro");

        // when
        let result = parse(&string);

        // then
        assert_eq!(result, String::from("6:jotaro"));
    }
}