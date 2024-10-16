// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)

// I AM DONE

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq_pass() {
        assert_eq!(2 + 2, 4); // цей тест пройде
    }

    #[test]
    fn you_can_assert_eq_fail() {
        assert_eq!(2 + 2, 5); // цей тест провалиться
    }
}

