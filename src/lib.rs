#![no_std]

#[cfg(test)]
mod tests {
    #[test]
    fn successfull_test() {
        assert_eq!(true, true);
    }

    #[test]
    fn failing_test() {
        assert_eq!(true, false);
    }
}
