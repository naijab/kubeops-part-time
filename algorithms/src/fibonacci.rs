pub fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

#[cfg(test)]
mod fibonacci_test {
    use super::*;

    #[test]
    fn it_2_should_return_1_fibonacci() {
        let actual = fibonacci(2);
        assert_eq!(actual, 1);
    }

    #[test]
    fn it_9_should_return_34_fibonacci() {
        let actual = fibonacci(9);
        assert_eq!(actual, 34);
    }
}
