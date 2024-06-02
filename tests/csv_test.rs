#[cfg(test)]
mod csv_tests {
    use json::*;
    use mawu::testing;
    use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn tester() {
        testing();
        assert!(true);
    }
}
