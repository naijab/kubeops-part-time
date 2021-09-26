pub fn is_palindrome(input: &str) -> bool {
    if input.len() == 0 {
        return true;
    }

    let mut first = 0;
    let mut last = input.len() - 1;

    let chars = input.as_bytes().to_owned();

    while first < last {
        if chars[first] != chars[last] {
            return false;
        }

        first += 1;
        last -= 1;
    }

    return true;
}

#[cfg(test)]
mod palindrome_test {
    use super::*;

    #[test]
    fn it_radar_is_palindrome() {
        let actual = is_palindrome("radar");
        assert_eq!(actual, true);
    }

    #[test]
    fn it_12321_is_palindrome() {
        let actual = is_palindrome("12321");
        assert_eq!(actual, true);
    }

    #[test]
    fn it_abbc_is_not_palindrome() {
        let actual = is_palindrome("abbc");
        assert_eq!(actual, false);
    }

    #[test]
    fn it_15621_is_not_palindrome() {
        let actual = is_palindrome("15621");
        assert_eq!(actual, false);
    }
}
