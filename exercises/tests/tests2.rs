// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        fn add_two(a: i32) -> i32 {
            a + 2
        }
        assert_eq!(4, add_two(2));
    }
}
