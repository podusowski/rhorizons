use std::num::ParseIntError;

use thiserror::Error;

/// Planet, natural satellite, spacecraft, Sun, barycenter, or other objects
/// having pre-computed trajectories.
///
/// https://ssd.jpl.nasa.gov/horizons/manual.html#defs
#[derive(Debug, PartialEq, Eq)]
pub struct MajorBody {
    id: i32,
    name: String,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum MajorBodyParseError {
    #[error("invalid id")]
    InvalidId(#[source] ParseIntError),
}

/// Similar to `str::split_at`, but instead panicking, it tries returning what
/// is possible.
fn take_or_empty(value: &str, n: usize) -> (&str, &str) {
    if value.len() > n {
        (&value[..n], &value[n..])
    } else {
        (value, "")
    }
}

impl TryFrom<&str> for MajorBody {
    type Error = MajorBodyParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // It seems (yes, I haven't found any specs) that Horizons formats its
        // result as fixed sized tables. Even if name exceeds the size of the
        // column, it gets truncated.

        let (id, value) = take_or_empty(value, 9);
        let (name, value) = take_or_empty(value, 35);

        Ok(Self {
            id: id.trim().parse().map_err(MajorBodyParseError::InvalidId)?,
            name: name.trim().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    use crate::parsing::MajorBodyParseError;

    use super::*;

    #[test]
    fn check_take_or_empty() {
        assert_eq!(("a", ""), take_or_empty("a", 1));
        assert_eq!(("", "a"), take_or_empty("a", 0));
        assert_eq!(("ab", "cd"), take_or_empty("abcd", 2));
        assert_eq!(("ab", ""), take_or_empty("ab", 4));
    }

    #[test]
    fn reading_major_bodies() {
        assert_eq!(
            MajorBody {
                id: 0,
                name: "Solar System Barycenter".to_string()
            },
            MajorBody::try_from("        0  Solar System Barycenter                         SSB")
                .unwrap()
        );

        assert_eq!(
            MajorBody {
                id: 699,
                name: "Saturn".to_string()
            },
            MajorBody::try_from("      699  Saturn").unwrap()
        );

        assert_eq!(
            MajorBody {
                id: -78000,
                name: "Chang'e_5-T1_booster (spacecraft)".to_string()
            },
            MajorBody::try_from(
                "  -78000  Chang'e_5-T1_booster (spacecraft)  WE0913A      2014-065B"
            )
            .unwrap()
        );
    }

    #[test]
    fn error_cases_when_parsing_major_bodies() {
        assert!(matches!(
            MajorBody::try_from("****************").unwrap_err(),
            MajorBodyParseError::InvalidId(ParseIntError { .. })
        ));

        assert!(matches!(
            MajorBody::try_from("").unwrap_err(),
            MajorBodyParseError::InvalidId(ParseIntError { .. })
        ));
    }
}
