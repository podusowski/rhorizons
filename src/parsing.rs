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

impl TryFrom<&str> for MajorBody {
    type Error = MajorBodyParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // It seems (yes, I haven't found any specs) that Horizons formats its
        // result as fixed sized tables. Even if name exceeds the size of the
        // column, it gets truncated.
        Ok(Self {
            id: value
                .get(0..9)
                .unwrap_or("none")
                .trim()
                .parse()
                .map_err(MajorBodyParseError::InvalidId)?,
            name: value.get(11..15).unwrap_or_default().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    use crate::parsing::MajorBodyParseError;

    use super::MajorBody;

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
