use thiserror::Error;

/// Similar to `str::split_at`, but instead panicking, it tries returning what
/// is possible.
pub fn take_or_empty(value: &str, n: usize) -> (&str, &str) {
    if value.len() > n {
        (&value[..n], &value[n..])
    } else {
        (value, "")
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
#[error("'{0}' does not contain expected prefix")]
pub struct TakeExpectingError<'a>(&'a str);

pub fn take_expecting<'a, 'b>(
    value: &'a str,
    expected: &'b str,
) -> Result<&'a str, TakeExpectingError<'a>> {
    let (prefix, rest) = (&value[..expected.len()], &value[expected.len()..]);
    if prefix == expected {
        Ok(rest)
    } else {
        Err(TakeExpectingError(value))
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

    #[test]
    fn test_take_expecting() {
        assert_eq!(Ok("b"), take_expecting("ab", "a"));
        assert_eq!(Err(TakeExpectingError("ba")), take_expecting("ba", "a"));
    }
}
