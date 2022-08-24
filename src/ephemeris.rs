use crate::utilities::{take_expecting, take_or_empty};

/// Position (in km) and velocity (in km/s) of a body.
#[derive(Debug, PartialEq)]
pub struct EphemerisItem {
    pub position: [f32; 3],
    pub velocity: [f32; 3],
}

enum EphemerisParserState {
    WaitingForSoe,
    Date,
    Position,
    Velocity {
        position: [f32; 3],
    },
    Other {
        position: [f32; 3],
        velocity: [f32; 3],
    },
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
                        // TODO: Don't panic.
                        let line = take_expecting(line, " X =").unwrap();
                        let (x, line) = take_or_empty(line, 22);

                        let line = take_expecting(line, " Y =").unwrap();
                        let (y, line) = take_or_empty(line, 22);

                        let line = take_expecting(line, " Z =").unwrap();
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
                        // TODO: Don't panic.
                        let line = take_expecting(line, " VX=").unwrap();
                        let (vx, line) = take_or_empty(line, 22);

                        let line = take_expecting(line, " VY=").unwrap();
                        let (vy, line) = take_or_empty(line, 22);

                        let line = take_expecting(line, " VZ=").unwrap();
                        let (vz, _) = take_or_empty(line, 22);

                        self.state = EphemerisParserState::Other {
                            position,
                            velocity: [
                                vx.trim().parse::<f32>().unwrap(),
                                vy.trim().parse::<f32>().unwrap(),
                                vz.trim().parse::<f32>().unwrap(),
                            ],
                        };
                    }
                    EphemerisParserState::Other { position, velocity } => {
                        self.state = EphemerisParserState::Date;
                        return Some(EphemerisItem { position, velocity });
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
    use super::*;

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
                ],

                velocity: [
                    -3.362664133558439E-01,
                    1.344100266143978E-02,
                    -5.030275220358716E-03
                ]
            },
            ephem[0]
        );
    }
}
