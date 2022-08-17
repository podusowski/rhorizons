use std::num::ParseIntError;

use thiserror::Error;

/// Planet, natural satellite, spacecraft, Sun, barycenter, or other objects
/// having pre-computed trajectories.
///
/// <https://ssd.jpl.nasa.gov/horizons/manual.html#defs>
#[derive(Debug, PartialEq, Eq)]
pub struct MajorBody {
    pub id: i32,
    pub name: String,
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
        let (name, _) = take_or_empty(value, 35);

        Ok(Self {
            id: id.trim().parse().map_err(MajorBodyParseError::InvalidId)?,
            name: name.trim().to_string(),
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct EphemerisItem {
    position: [f32; 3],
}

enum EphemerisParserState {
    WaitingForSoe,
    Date,
    Position,
    Velocity { position: [f32; 3] },
    Other { position: [f32; 3] },
    End,
}

pub struct EphemerisParser<'a, Input: Iterator<Item = &'a str>> {
    state: EphemerisParserState,
    input: Input,
}

impl<'a, Input: Iterator<Item = &'a str>> EphemerisParser<'a, Input> {
    pub fn parse(input: Input) -> Self {
        Self {
            state: EphemerisParserState::WaitingForSoe,
            input,
        }
    }
}

impl<'a, Input: Iterator<Item = &'a str>> Iterator for EphemerisParser<'a, Input> {
    type Item = EphemerisItem;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(line) = self.input.next() {
                match self.state {
                    EphemerisParserState::WaitingForSoe => {
                        if line == "$$SOE" {
                            self.state = EphemerisParserState::Date;
                        }
                    }
                    EphemerisParserState::Date => {
                        if line == "$$EOE" {
                            self.state = EphemerisParserState::End;
                        } else {
                            self.state = EphemerisParserState::Position;
                        }
                    }
                    EphemerisParserState::Position => {
                        let (x, line) = take_or_empty(line, 4);
                        assert_eq!(x, " X =");
                        let (x, line) = take_or_empty(line, 22);

                        let (y, line) = take_or_empty(line, 4);
                        assert_eq!(y, " Y =");
                        let (y, line) = take_or_empty(line, 22);

                        let (z, line) = take_or_empty(line, 4);
                        assert_eq!(z, " Z =");
                        let (z, _) = take_or_empty(line, 22);

                        self.state = EphemerisParserState::Velocity {
                            position: [
                                x.trim().parse::<f32>().unwrap(),
                                y.trim().parse::<f32>().unwrap(),
                                z.trim().parse::<f32>().unwrap(),
                            ],
                        };
                    }
                    EphemerisParserState::Velocity { position } => {
                        self.state = EphemerisParserState::Other { position };
                    }
                    EphemerisParserState::Other { position } => {
                        self.state = EphemerisParserState::Date;
                        return Some(EphemerisItem { position });
                    }
                    EphemerisParserState::End => {
                        // Should we drain input iterator?
                        return None;
                    }
                }
            } else {
                // Input iterator is drained. Nothing to do.
                return None;
            }
        }
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

    #[test]
    fn test_parsing_ephemeris() {
        let data = include_str!("ephem.txt");
        let ephem: Vec<_> = EphemerisParser::parse(data.lines()).collect();
        assert_eq!(4, ephem.len());
        // TODO: This will probably fail intermittently due to float comparison.
        assert_eq!(
            EphemerisItem {
                position: [
                    1.870010427985840E+02,
                    2.484687803242536E+03,
                    -5.861602653492581E+03
                ]
            },
            ephem[0]
        );
    }
}
