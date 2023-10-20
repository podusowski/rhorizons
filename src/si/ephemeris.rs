use crate::utilities::{take_expecting, take_or_empty};

use uom::si::f32::{Length, Velocity, Angle, AngularVelocity, Time};
use uom::si::{length, velocity, angle, angular_velocity, time};

/// Position and velocity of a body. Units are SI-based
///
/// | Horizons Symbol | Meaning                                         |
/// |-----------------|-------------------------------------------------|
/// | X               | X-component of position vector                  |
/// | Y               | Y-component of position vector                  |
/// | Z               | Z-component of position vector                  |
/// | VX              | X-component of velocity vector                  |
/// | VY              | Y-component of velocity vector                  |
/// | VZ              | Z-component of velocity vector                  |
/// | LT              | One-way down-leg Newtonian light-time           |
/// | RG              | Range; distance from coordinate center          |
/// | RR              | Range-rate; radial velocity wrt coord. center   |
#[derive(Debug, PartialEq)]
pub struct EphemerisVectorItem {
    /// Position of the moving body relative to the Sun
    ///
    /// [x, y, z]
    pub position: [Length; 3],

    /// Velocity of the moving body relative to the Sun
    ///
    /// [v_x, v_y, v_z]
    pub velocity: [Velocity; 3],
}

/// Orbital Elements of a body. Units are SI-based
///
/// | Horizons Symbol | Meaning                     | Mathematical Symbol |
/// |-----------------|-----------------------------|---------------------|
/// | EC              | Eccentricity                | *e*                 |
/// | QR              | Periapsis distance          | *q*                 |
/// | IN              | Inclination w.r.t X-Y plane | *i*                 |
/// | OM              | Longitude of Ascending Node | Ω (Omega)           |
/// | W               | Argument of Perifocus       | *ω*                 |
/// | Tp              | Time of periapsis           |                     |
/// | N               | Mean motion                 | *n*                 |
/// | MA              | Mean anomaly                | *M*                 |
/// | TA              | True anomaly                | *ν*, nu             |
/// | A               | Semi-major axis             | *a*                 |
/// | AD              | Apoapsis distance           |                     |
/// | PR              | Sidereal orbit period       |                     |
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
    /// Distance from the center to the nearest point of the orbit
    ///
    /// See <https://en.wikipedia.org/wiki/Apsis>
    pub periapsis_distance: Length,
    /// Tilt of the orbit
    ///
    /// In reference to the X-Y plane  
    /// For futher information see <https://en.wikipedia.org/wiki/Inclination>
    pub inclination: Angle,

    /// The point, were the orbit crosses the reference plane (X-Y plane) from south to north
    ///
    /// <https://en.wikipedia.org/wiki/Longitude_of_the_ascending_node>
    pub longitude_of_ascending_node: Angle,
    /// Angle of the periapsis to the ascending node, in the direction of motion.
    ///
    /// <https://en.wikipedia.org/wiki/Argument_of_periapsis>
    pub argument_of_perifocus: Angle,
    /// The timestamp (Julian Day Number) at which the body reaches the periapsis of the orbit
    ///
    /// <https://en.wikipedia.org/wiki/Apsis#Time_of_perihelion>
    pub time_of_periapsis: Time,

    /// The angular speed of a body to complete one orbit
    ///
    /// Assumes constant speed in a circular orbit.  
    /// <https://en.wikipedia.org/wiki/Mean_motion>
    pub mean_motion: AngularVelocity,
    /// Orbital distance from the periapsis to the moving body.
    ///
    /// The angle is in reference to a circular orbit.  
    /// <https://en.wikipedia.org/wiki/Mean_anomaly>
    pub mean_anomaly: Angle,
    /// Angle between the moving body and the periapsis of the orbit.
    ///
    /// The angle is defined in relation to the main focus point.  
    /// <https://en.wikipedia.org/wiki/True_anomaly>
    pub true_anomaly: Angle,

    /// The sum of the periapsis and apoapsis distances divided by two
    ///
    /// <https://en.wikipedia.org/wiki/Semimajor_axis>
    pub semi_major_axis: Length,
    /// Distance from the center to the farthest point of the orbit
    ///
    /// <https://en.wikipedia.org/wiki/Apsis>
    pub apoapsis_distance: Length,
    /// Time to complete on orbit in seconds
    ///
    /// Sidereal refers to the default period of an orbit.  
    /// <https://en.wikipedia.org/wiki/Orbital_period>
    pub siderral_orbit_period: Time,
}

enum EphemerisVectorParserState {
    WaitingForSoe,
    WaitingForDate,
    WaitingForPosition,
    Position {
        position: [Length; 3],
    },
    Complete {
        position: [Length; 3],
        velocity: [Velocity; 3],
    },
    End,
}

enum EphemerisOrbitalElementsParserState {
    WaitingForSoe,
    WaitingForDate,
    WaitingForFirstRow,
    FirstRow {
        eccentricity: f32,
        periapsis_distance: Length,
        inclination: Angle,
    },
    SecondRow {
        eccentricity: f32,
        periapsis_distance: Length,
        inclination: Angle,

        longitude_of_ascending_node: Angle,
        argument_of_perifocus: Angle,
        time_of_periapsis: Time,
    },
    ThirdRow {
        eccentricity: f32,
        periapsis_distance: Length,
        inclination: Angle,

        longitude_of_ascending_node: Angle,
        argument_of_perifocus: Angle,
        time_of_periapsis: Time,

        mean_motion: AngularVelocity,
        mean_anomaly: Angle,
        true_anomaly: Angle,
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
                                Length::new::<length::kilometer>(x.trim().parse::<f32>().unwrap()),
                                Length::new::<length::kilometer>(y.trim().parse::<f32>().unwrap()),
                                Length::new::<length::kilometer>(z.trim().parse::<f32>().unwrap()),
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
                                Velocity::new::<velocity::kilometer_per_second>(vx.trim().parse::<f32>().unwrap()),
                                Velocity::new::<velocity::kilometer_per_second>(vy.trim().parse::<f32>().unwrap()),
                                Velocity::new::<velocity::kilometer_per_second>(vz.trim().parse::<f32>().unwrap()),
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
                            periapsis_distance: Length::new::<length::kilometer>(periapsis_distance.trim().parse::<f32>().unwrap()),
                            inclination: Angle::new::<angle::degree>(inclination.trim().parse::<f32>().unwrap()),
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
                            longitude_of_ascending_node: Angle::new::<angle::degree>(longitude_of_ascending_node
                                .trim()
                                .parse::<f32>()
                                .unwrap()),
                            argument_of_perifocus: Angle::new::<angle::degree>(argument_of_perifocus
                                .trim()
                                .parse::<f32>()
                                .unwrap()),
                            time_of_periapsis: Time::new::<time::day>(time_of_periapsis.trim().parse::<f32>().unwrap()),
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
                            mean_motion: AngularVelocity::new::<angular_velocity::degree_per_second>(mean_motion.trim().parse::<f32>().unwrap()),
                            mean_anomaly: Angle::new::<angle::degree>(mean_anomaly.trim().parse::<f32>().unwrap()),
                            true_anomaly: Angle::new::<angle::degree>(true_anomaly.trim().parse::<f32>().unwrap()),
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
                            semi_major_axis: Length::new::<length::kilometer>(semi_major_axis.trim().parse::<f32>().unwrap()),
                            apoapsis_distance: Length::new::<length::kilometer>(apoapsis_distance.trim().parse::<f32>().unwrap()),
                            siderral_orbit_period: Time::new::<time::second>(siderral_orbit_period
                                .trim()
                                .parse::<f32>()
                                .unwrap()),
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
        let data = include_str!("../vector.txt");
        let ephem: Vec<_> = EphemerisVectorParser::parse(data.lines()).collect();
        assert_eq!(4, ephem.len());
        // TODO: This will probably fail intermittently due to float comparison.
        assert_eq!(
            EphemerisVectorItem {
                position: [
                    Length::new::<length::kilometer>(1.870010427985840E+02),
                    Length::new::<length::kilometer>(2.484687803242536E+03),
                    Length::new::<length::kilometer>(-5.861602653492581E+03)
                ],

                velocity: [
                    Velocity::new::<velocity::kilometer_per_second>(-3.362664133558439E-01),
                    Velocity::new::<velocity::kilometer_per_second>(1.344100266143978E-02),
                    Velocity::new::<velocity::kilometer_per_second>(-5.030275220358716E-03)
                ]
            },
            ephem[0]
        );
    }

    #[test]
    fn test_parsing_ephemeris_orbital_elements() {
        let data = include_str!("../orbital_elements.txt");
        let ephem: Vec<_> = EphemerisOrbitalElementsParser::parse(data.lines()).collect();
        assert_eq!(4, ephem.len());
        // TODO: This will probably fail intermittently due to float comparison.
        assert_eq!(
            EphemerisOrbitalElementsItem {
                eccentricity: 1.711794334680415E-02,
                periapsis_distance: Length::new::<length::kilometer>(1.469885520304013E+08),
                inclination: Angle::new::<angle::degree>(3.134746902320420E-03),

                longitude_of_ascending_node: Angle::new::<angle::degree>(1.633896137466430E+02),
                argument_of_perifocus: Angle::new::<angle::degree>(3.006492364709574E+02),
                time_of_periapsis: Time::new::<time::day>(2459584.392523936927),

                mean_motion: AngularVelocity::new::<angular_velocity::degree_per_second>(1.141316101270797E-05),
                mean_anomaly: Angle::new::<angle::degree>(1.635515780663357E+02),
                true_anomaly: Angle::new::<angle::degree>(1.640958153023696E+02),

                semi_major_axis: Length::new::<length::kilometer>(1.495485150384278E+08),
                apoapsis_distance: Length::new::<length::kilometer>(1.521084780464543E+08),
                siderral_orbit_period: Time::new::<time::second>(3.154253230977451E+07),
            },
            ephem[0]
        );
    }
}
