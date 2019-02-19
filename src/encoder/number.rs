pub fn parse(num: i64) -> String {
    let mut result = String::from("i");
    result.push_str(&num.to_string());
    result.push('e');
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_given_number() {
        // given
        let num: i64 = 420;

        // when
        let result = parse(num);

        // then
        assert_eq!(result, String::from("i420e"));
    }
}