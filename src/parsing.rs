use std::num::ParseIntError;

use thiserror::Error;

/// Planet, natural satellite, spacecraft, Sun, barycenter, or other objects
/// having pre-computed trajectories.
///
/// https://ssd.jpl.nasa.gov/horizons/manual.html#defs
#[derive(Debug, PartialEq)]
pub struct MajorBody {
    id: i32,
}

#[derive(Error, Debug)]
pub enum MajorBodyParseError {
    #[error("invalid id")]
    InvalidId(#[source] ParseIntError),
}

impl TryFrom<&str> for MajorBody {
    type Error = MajorBodyParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            // TODO: Emit some meaningful error.
            id: value
                .get(0..9)
                .unwrap_or("none")
                .trim()
                .parse()
                .map_err(MajorBodyParseError::InvalidId)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MajorBody;

    #[test]
    fn reading_major_bodies() {
        assert_eq!(
            MajorBody { id: 0 },
            MajorBody::try_from("        0  Solar System Barycenter                         SSB")
                .unwrap()
        );
    }
}
