use crate::utilities::take_or_empty;
use std::num::ParseIntError;
use thiserror::Error;

/// Planet, natural satellite, spacecraft, Sun, barycenter, or other objects
/// having pre-computed trajectories.
///
/// <https://ssd.jpl.nasa.gov/horizons/manual.html#defs>
///
/// Example
/// ```
/// # use rhorizons::MajorBody;
/// let mb = MajorBody {
///     id: 399,
///     name: "Earth".to_string()
/// };
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct MajorBody {
    /// Id of the major body
    pub id: i32,
    /// Name of the major body (e.g. Earth)
    pub name: String,
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

        let (id, value) = take_or_empty(value, 9);
        let (name, _) = take_or_empty(value, 35);

        Ok(Self {
            id: id.trim().parse().map_err(MajorBodyParseError::InvalidId)?,
            name: name.trim().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::ParseIntError;

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
