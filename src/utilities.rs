/// Similar to `str::split_at`, but instead panicking, it tries returning what
/// is possible.
pub fn take_or_empty(value: &str, n: usize) -> (&str, &str) {
    if value.len() > n {
        (&value[..n], &value[n..])
    } else {
        (value, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_take_or_empty() {
        assert_eq!(("a", ""), take_or_empty("a", 1));
        assert_eq!(("", "a"), take_or_empty("a", 0));
        assert_eq!(("ab", "cd"), take_or_empty("abcd", 2));
        assert_eq!(("ab", ""), take_or_empty("ab", 4));
    }
}
