/// Similar to `str::split_at`, but instead panicking, it tries returning what
/// is possible.
pub fn take_or_empty(value: &str, n: usize) -> (&str, &str) {
    if value.len() > n {
        (&value[..n], &value[n..])
    } else {
        (value, "")
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
                ]
            },
            ephem[0]
        );
    }
}
