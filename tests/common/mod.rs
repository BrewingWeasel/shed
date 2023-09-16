#[macro_export]
macro_rules! run_test {
    ($original:expr, $expression:expr, $output:expr) => {
        assert_eq!(
            String::from($output),
            parse(
                vec![$expression.to_string()],
                Config { quiet: false },
                String::from($original),
            )
        )
    };
}
