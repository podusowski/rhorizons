use crate::utilities::{take_expecting, take_or_empty};

/// Position (in km) and velocity (in km/s) of a body.
#[derive(Debug, PartialEq)]
pub struct EphemerisVectorItem {
    pub position: [f32; 3],
    pub velocity: [f32; 3],
}

enum EphemerisVectorParserState {
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

pub struct EphemerisVectorParser<'a, Input: Iterator<Item = &'a str>> {
    state: EphemerisVectorParserState,
    input: Input,
}

impl<'a, Input: Iterator<Item = &'a str>> EphemerisVectorParser<'a, Input> {
    pub fn parse(input: Input) -> Self {
        Self {
            state: EphemerisVectorParserState::WaitingForSoe,
            input,
        }
    }
}

impl<'a, Input: Iterator<Item = &'a str>> Iterator for EphemerisVectorParser<'a, Input> {
    type Item = EphemerisVectorItem;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(line) = self.input.next() {
                match self.state {
                    EphemerisVectorParserState::WaitingForSoe => {
                        if line == "$$SOE" {
                            self.state = EphemerisVectorParserState::Date;
                        }
                    }
                    EphemerisVectorParserState::Date => {
                        if line == "$$EOE" {
                            self.state = EphemerisVectorParserState::End;
                        } else {
                            self.state = EphemerisVectorParserState::Position;
                        }
                    }
                    EphemerisVectorParserState::Position => {
                        // TODO: Don't panic.
                        let line = take_expecting(line, " X =").unwrap();
                        let (x, line) = take_or_empty(line, 22);

                        let line = take_expecting(line, " Y =").unwrap();
                        let (y, line) = take_or_empty(line, 22);

                        let line = take_expecting(line, " Z =").unwrap();
                        let (z, _) = take_or_empty(line, 22);

                        self.state = EphemerisVectorParserState::Velocity {
                            position: [
                                x.trim().parse::<f32>().unwrap(),
                                y.trim().parse::<f32>().unwrap(),
                                z.trim().parse::<f32>().unwrap(),
                            ],
                        };
                    }
                    EphemerisVectorParserState::Velocity { position } => {
                        // TODO: Don't panic.
                        let line = take_expecting(line, " VX=").unwrap();
                        let (vx, line) = take_or_empty(line, 22);

                        let line = take_expecting(line, " VY=").unwrap();
                        let (vy, line) = take_or_empty(line, 22);

                        let line = take_expecting(line, " VZ=").unwrap();
                        let (vz, _) = take_or_empty(line, 22);

                        self.state = EphemerisVectorParserState::Other {
                            position,
                            velocity: [
                                vx.trim().parse::<f32>().unwrap(),
                                vy.trim().parse::<f32>().unwrap(),
                                vz.trim().parse::<f32>().unwrap(),
                            ],
                        };
                    }
                    EphemerisVectorParserState::Other { position, velocity } => {
                        self.state = EphemerisVectorParserState::Date;
                        return Some(EphemerisVectorItem { position, velocity });
                    }
                    EphemerisVectorParserState::End => {
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
        let data = include_str!("vector.txt");
        let ephem: Vec<_> = EphemerisVectorParser::parse(data.lines()).collect();
        assert_eq!(4, ephem.len());
        // TODO: This will probably fail intermittently due to float comparison.
        assert_eq!(
            EphemerisVectorItem {
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
