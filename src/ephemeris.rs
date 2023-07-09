use crate::utilities::{take_expecting, take_or_empty};

/// Position (in km) and velocity (in km/s) of a body.
///
/// | Horizons Symbol | Meaning                                         | Unit                  |
/// |-----------------|-------------------------------------------------|-----------------------|
/// | X               | X-component of position vector                  | km                    |
/// | Y               | Y-component of position vector                  | km                    |
/// | Z               | Z-component of position vector                  | km                    |
/// | VX              | X-component of velocity vector                  | km/sec                |
/// | VY              | Y-component of velocity vector                  | km/sec                |
/// | VZ              | Z-component of velocity vector                  | km/sec                |
/// | LT              | One-way down-leg Newtonian light-time           | sec                   |
/// | RG              | Range; distance from coordinate center          | km                    |
/// | RR              | Range-rate; radial velocity wrt coord. center   | km/sec                |
#[derive(Debug, PartialEq)]
pub struct EphemerisVectorItem {
    /// Position int km of the moving body relative to the Sun
    ///
    /// [x, y, z]
    pub position: [f32; 3],

    /// Velocity in km/s of the moving body relative to the Sun
    ///
    /// [v_x, v_y, v_z]
    pub velocity: [f32; 3],
}

/// Orbital Elements of a body. Units are km, s and degrees
///
/// | Horizons Symbol | Meaning                     | Mathematical Symbol | Unit                |
/// |-----------------|-----------------------------|---------------------|---------------------|
/// | EC              | Eccentricity                | *e*                 |                     |
/// | QR              | Periapsis distance          | *q*                 | km                  |
/// | IN              | Inclination w.r.t X-Y plane | *i*                 | degrees             |
/// | OM              | Longitude of Ascending Node | Ω (Omega)           | degrees             |
/// | W               | Argument of Perifocus       | *ω*                 | degrees             |
/// | Tp              | Time of periapsis           |                     | Julian Day Number   |
/// | N               | Mean motion                 | *n*                 | degrees/sec         |
/// | MA              | Mean anomaly                | *M*                 | degrees             |
/// | TA              | True anomaly                | *ν*, nu             | degrees             |
/// | A               | Semi-major axis             | *a*                 | km                  |
/// | AD              | Apoapsis distance           |                     | km                  |
/// | PR              | Sidereal orbit period       |                     | sec                 |
///
/// For a detailed explenation of keplarian orbital elements, visit [Wikipedia](https://en.wikipedia.org/wiki/Orbital_elements)
#[derive(Debug, PartialEq)]
pub struct EphemerisOrbitalElementsItem {
    /// Describes the "roundness" of the orbit.
    ///
    /// Value of 0 means a circle, everything until 1 is an eliptic orbit.  
    /// A value of 1 is a parabolic trajectory and everythin greater 1 a hyperbolic trajectory.  
    /// See <https://en.wikipedia.org/wiki/Eccentricity_(orbit)>
    pub eccentricity: f32,
    /// Distance from the center to the nearest point of the orbit in kilometer (km)
    ///
    /// See <https://en.wikipedia.org/wiki/Apsis>
    pub periapsis_distance: f32,
    /// Tilt of the orbit
    ///
    /// Expressed in degrees in reference to the X-Y plane  
    /// For futher information see <https://en.wikipedia.org/wiki/Inclination>
    pub inclination: f32,

    /// The point, were the orbit crosses the reference plane (X-Y plane) from south to north
    ///
    /// The unit of this value is in degrees.  
    /// <https://en.wikipedia.org/wiki/Longitude_of_the_ascending_node>
    pub longitude_of_ascending_node: f32,
    /// Angle in degrees of the periapsis to the ascending node, in the direction of motion.
    ///
    /// <https://en.wikipedia.org/wiki/Argument_of_periapsis>
    pub argument_of_perifocus: f32,
    /// The timestamp (Julian Day Number) at which the body reaches the periapsis of the orbit
    ///
    /// <https://en.wikipedia.org/wiki/Apsis#Time_of_perihelion>
    pub time_of_periapsis: f32,

    /// The angular speed (degrees/sec) of a body to complete one orbit
    ///
    /// Assumes constant speed in a circular orbit.  
    /// <https://en.wikipedia.org/wiki/Mean_motion>
    pub mean_motion: f32,
    /// Orbital distance from the periapsis to the moving body.
    ///
    /// The angle in degrees is in reference to a circular orbit.  
    /// <https://en.wikipedia.org/wiki/Mean_anomaly>
    pub mean_anomaly: f32,
    /// Angle in degrees between the moving body and the periapsis of the orbit.
    ///
    /// The angle is defined in relation to the main focus point.  
    /// <https://en.wikipedia.org/wiki/True_anomaly>
    pub true_anomaly: f32,

    /// The sum of the periapsis and apoapsis distances divided by two in kilometer (km)
    ///
    /// <https://en.wikipedia.org/wiki/Semimajor_axis>
    pub semi_major_axis: f32,
    /// Distance from the center to the farthest point of the orbit in kilometer (km)
    ///
    /// <https://en.wikipedia.org/wiki/Apsis>
    pub apoapsis_distance: f32,
    /// Time to complete on orbit in seconds
    ///
    /// Sidereal refers to the default period of an orbit.  
    /// <https://en.wikipedia.org/wiki/Orbital_period>
    pub siderral_orbit_period: f32,
}

enum EphemerisVectorParserState {
    WaitingForSoe,
    WaitingForDate,
    WaitingForPosition,
    Position {
        position: [f32; 3],
    },
    Complete {
        position: [f32; 3],
        velocity: [f32; 3],
    },
    End,
}

enum EphemerisOrbitalElementsParserState {
    WaitingForSoe,
    WaitingForDate,
    WaitingForFirstRow,
    FirstRow {
        eccentricity: f32,
        periapsis_distance: f32,
        inclination: f32,
    },
    SecondRow {
        eccentricity: f32,
        periapsis_distance: f32,
        inclination: f32,

        longitude_of_ascending_node: f32,
        argument_of_perifocus: f32,
        time_of_periapsis: f32,
    },
    ThirdRow {
        eccentricity: f32,
        periapsis_distance: f32,
        inclination: f32,

        longitude_of_ascending_node: f32,
        argument_of_perifocus: f32,
        time_of_periapsis: f32,

        mean_motion: f32,
        mean_anomaly: f32,
        true_anomaly: f32,
    },
    End,
}

pub struct EphemerisVectorParser<'a, Input: Iterator<Item = &'a str>> {
    state: EphemerisVectorParserState,
    input: Input,
}

pub struct EphemerisOrbitalElementsParser<'a, Input: Iterator<Item = &'a str>> {
    state: EphemerisOrbitalElementsParserState,
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

impl<'a, Input: Iterator<Item = &'a str>> EphemerisOrbitalElementsParser<'a, Input> {
    pub fn parse(input: Input) -> Self {
        Self {
            state: EphemerisOrbitalElementsParserState::WaitingForSoe,
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
                            self.state = EphemerisVectorParserState::WaitingForDate;
                        }
                    }
                    EphemerisVectorParserState::WaitingForDate => {
                        if line == "$$EOE" {
                            self.state = EphemerisVectorParserState::End;
                        } else {
                            self.state = EphemerisVectorParserState::WaitingForPosition;
                        }
                    }
                    EphemerisVectorParserState::WaitingForPosition => {
                        // TODO: Don't panic.
                        let line = take_expecting(line, " X =").unwrap();
                        let (x, line) = take_or_empty(line, 22);

                        let line = take_expecting(line, " Y =").unwrap();
                        let (y, line) = take_or_empty(line, 22);

                        let line = take_expecting(line, " Z =").unwrap();
                        let (z, _) = take_or_empty(line, 22);

                        self.state = EphemerisVectorParserState::Position {
                            position: [
                                x.trim().parse::<f32>().unwrap(),
                                y.trim().parse::<f32>().unwrap(),
                                z.trim().parse::<f32>().unwrap(),
                            ],
                        };
                    }
                    EphemerisVectorParserState::Position { position } => {
                        // TODO: Don't panic.
                        let line = take_expecting(line, " VX=").unwrap();
                        let (vx, line) = take_or_empty(line, 22);

                        let line = take_expecting(line, " VY=").unwrap();
                        let (vy, line) = take_or_empty(line, 22);

                        let line = take_expecting(line, " VZ=").unwrap();
                        let (vz, _) = take_or_empty(line, 22);

                        self.state = EphemerisVectorParserState::Complete {
                            position,
                            velocity: [
                                vx.trim().parse::<f32>().unwrap(),
                                vy.trim().parse::<f32>().unwrap(),
                                vz.trim().parse::<f32>().unwrap(),
                            ],
                        };
                    }
                    // Would parse third line and then return Item => ignores third line and returns directly
                    EphemerisVectorParserState::Complete { position, velocity } => {
                        self.state = EphemerisVectorParserState::WaitingForDate;
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

impl<'a, Input: Iterator<Item = &'a str>> Iterator for EphemerisOrbitalElementsParser<'a, Input> {
    type Item = EphemerisOrbitalElementsItem;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(line) = self.input.next() {
                match self.state {
                    EphemerisOrbitalElementsParserState::WaitingForSoe => {
                        if line == "$$SOE" {
                            self.state = EphemerisOrbitalElementsParserState::WaitingForDate;
                        }
                    }
                    EphemerisOrbitalElementsParserState::WaitingForDate => {
                        if line == "$$EOE" {
                            self.state = EphemerisOrbitalElementsParserState::End;
                        } else {
                            self.state = EphemerisOrbitalElementsParserState::WaitingForFirstRow;
                        }
                    }
                    EphemerisOrbitalElementsParserState::WaitingForFirstRow => {
                        let line = take_expecting(line, " EC=").unwrap();
                        let (eccentricity, line) = take_or_empty(line, 22);

                        let line = take_expecting(line, " QR=").unwrap();
                        let (periapsis_distance, line) = take_or_empty(line, 22);

                        let line = take_expecting(line, " IN=").unwrap();
                        let (inclination, _) = take_or_empty(line, 22);

                        self.state = EphemerisOrbitalElementsParserState::FirstRow {
                            eccentricity: eccentricity.trim().parse::<f32>().unwrap(),
                            periapsis_distance: periapsis_distance.trim().parse::<f32>().unwrap(),
                            inclination: inclination.trim().parse::<f32>().unwrap(),
                        };
                    }
                    EphemerisOrbitalElementsParserState::FirstRow {
                        eccentricity,
                        periapsis_distance,
                        inclination,
                    } => {
                        let line = take_expecting(line, " OM=").unwrap();
                        let (longitude_of_ascending_node, line) = take_or_empty(line, 22);

                        let line = take_expecting(line, " W =").unwrap();
                        let (argument_of_perifocus, line) = take_or_empty(line, 22);

                        let line = take_expecting(line, " Tp=").unwrap();
                        let (time_of_periapsis, _) = take_or_empty(line, 22);

                        self.state = EphemerisOrbitalElementsParserState::SecondRow {
                            eccentricity,
                            periapsis_distance,
                            inclination,
                            longitude_of_ascending_node: longitude_of_ascending_node
                                .trim()
                                .parse::<f32>()
                                .unwrap(),
                            argument_of_perifocus: argument_of_perifocus
                                .trim()
                                .parse::<f32>()
                                .unwrap(),
                            time_of_periapsis: time_of_periapsis.trim().parse::<f32>().unwrap(),
                        };
                    }
                    EphemerisOrbitalElementsParserState::SecondRow {
                        eccentricity,
                        periapsis_distance,
                        inclination,
                        longitude_of_ascending_node,
                        argument_of_perifocus,
                        time_of_periapsis,
                    } => {
                        let line = take_expecting(line, " N =").unwrap();
                        let (mean_motion, line) = take_or_empty(line, 22);

                        let line = take_expecting(line, " MA=").unwrap();
                        let (mean_anomaly, line) = take_or_empty(line, 22);

                        let line = take_expecting(line, " TA=").unwrap();
                        let (true_anomaly, _) = take_or_empty(line, 22);

                        self.state = EphemerisOrbitalElementsParserState::ThirdRow {
                            eccentricity,
                            periapsis_distance,
                            inclination,
                            longitude_of_ascending_node,
                            argument_of_perifocus,
                            time_of_periapsis,
                            mean_motion: mean_motion.trim().parse::<f32>().unwrap(),
                            mean_anomaly: mean_anomaly.trim().parse::<f32>().unwrap(),
                            true_anomaly: true_anomaly.trim().parse::<f32>().unwrap(),
                        };
                    }
                    // Parses last line and return Item
                    EphemerisOrbitalElementsParserState::ThirdRow {
                        eccentricity,
                        periapsis_distance,
                        inclination,
                        longitude_of_ascending_node,
                        argument_of_perifocus,
                        time_of_periapsis,
                        mean_motion,
                        mean_anomaly,
                        true_anomaly,
                    } => {
                        let line = take_expecting(line, " A =").unwrap();
                        let (semi_major_axis, line) = take_or_empty(line, 22);

                        let line = take_expecting(line, " AD=").unwrap();
                        let (apoapsis_distance, line) = take_or_empty(line, 22);

                        let line = take_expecting(line, " PR=").unwrap();
                        let (siderral_orbit_period, _) = take_or_empty(line, 22);

                        self.state = EphemerisOrbitalElementsParserState::WaitingForDate;
                        return Some(EphemerisOrbitalElementsItem {
                            eccentricity,
                            periapsis_distance,
                            inclination,
                            longitude_of_ascending_node,
                            argument_of_perifocus,
                            time_of_periapsis,
                            mean_motion,
                            mean_anomaly,
                            true_anomaly,
                            semi_major_axis: semi_major_axis.trim().parse::<f32>().unwrap(),
                            apoapsis_distance: apoapsis_distance.trim().parse::<f32>().unwrap(),
                            siderral_orbit_period: siderral_orbit_period
                                .trim()
                                .parse::<f32>()
                                .unwrap(),
                        });
                    }
                    EphemerisOrbitalElementsParserState::End => {
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
    fn test_parsing_ephemeris_vector() {
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

    #[test]
    fn test_parsing_ephemeris_orbital_elements() {
        let data = include_str!("orbital_elements.txt");
        let ephem: Vec<_> = EphemerisOrbitalElementsParser::parse(data.lines()).collect();
        assert_eq!(4, ephem.len());
        // TODO: This will probably fail intermittently due to float comparison.
        assert_eq!(
            EphemerisOrbitalElementsItem {
                eccentricity: 1.711794334680415E-02,
                periapsis_distance: 1.469885520304013E+08,
                inclination: 3.134746902320420E-03,

                longitude_of_ascending_node: 1.633896137466430E+02,
                argument_of_perifocus: 3.006492364709574E+02,
                time_of_periapsis: 2459584.392523936927,

                mean_motion: 1.141316101270797E-05,
                mean_anomaly: 1.635515780663357E+02,
                true_anomaly: 1.640958153023696E+02,

                semi_major_axis: 1.495485150384278E+08,
                apoapsis_distance: 1.521084780464543E+08,
                siderral_orbit_period: 3.154253230977451E+07,
            },
            ephem[0]
        );
    }
}
