use chrono::{DateTime, Utc};
use uom::si::f32::{Angle, AngularVelocity, Length, Time, Velocity};
use uom::si::{angle, angular_velocity, length, time, velocity};

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
    /// Timestamp of the entry in UTC
    pub time: DateTime<Utc>,

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
    /// Timestamp of the entry in UTC
    pub time: DateTime<Utc>,

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

impl From<crate::EphemerisVectorItem> for EphemerisVectorItem {
    fn from(item: crate::EphemerisVectorItem) -> Self {
        let position: Vec<Length> = item
            .position
            .into_iter()
            .map(Length::new::<length::kilometer>)
            .collect();
        let velocity: Vec<Velocity> = item
            .velocity
            .into_iter()
            .map(Velocity::new::<velocity::kilometer_per_second>)
            .collect();

        EphemerisVectorItem {
            time: item.time,
            position: position.try_into().unwrap(),
            velocity: velocity.try_into().unwrap(),
        }
    }
}

impl From<crate::EphemerisOrbitalElementsItem> for EphemerisOrbitalElementsItem {
    fn from(item: crate::EphemerisOrbitalElementsItem) -> Self {
        EphemerisOrbitalElementsItem {
            time: item.time,
            eccentricity: item.eccentricity,
            periapsis_distance: Length::new::<length::kilometer>(item.periapsis_distance),
            inclination: Angle::new::<angle::degree>(item.inclination),
            longitude_of_ascending_node: Angle::new::<angle::degree>(
                item.longitude_of_ascending_node,
            ),
            argument_of_perifocus: Angle::new::<angle::degree>(item.argument_of_perifocus),
            time_of_periapsis: Time::new::<time::day>(item.time_of_periapsis),
            mean_motion: AngularVelocity::new::<angular_velocity::degree_per_second>(
                item.mean_motion,
            ),
            mean_anomaly: Angle::new::<angle::degree>(item.mean_anomaly),
            true_anomaly: Angle::new::<angle::degree>(item.true_anomaly),
            semi_major_axis: Length::new::<length::kilometer>(item.semi_major_axis),
            apoapsis_distance: Length::new::<length::kilometer>(item.apoapsis_distance),
            siderral_orbit_period: Time::new::<time::second>(item.siderral_orbit_period),
        }
    }
}
/*
pub struct EphemerisVectorParser<'a, Input: Iterator<Item = &'a str>> {
    parser: crate::ephemeris::EphemerisVectorParser<'a, Input>,
}

pub struct EphemerisOrbitalElementsParser<'a, Input: Iterator<Item = &'a str>> {
    parser: crate::ephemeris::EphemerisOrbitalElementsParser<'a, Input>,
}

impl<'a, Input: Iterator<Item = &'a str>> EphemerisVectorParser<'a, Input> {
    pub fn parse(input: Input) -> Self {
        Self {
            parser: crate::ephemeris::EphemerisVectorParser::parse(input),
        }
    }
}

impl<'a, Input: Iterator<Item = &'a str>> EphemerisOrbitalElementsParser<'a, Input> {
    pub fn parse(input: Input) -> Self {
        Self {
            parser: crate::ephemeris::EphemerisOrbitalElementsParser::parse(input),
        }
    }
}

impl<'a, Input: Iterator<Item = &'a str>> Iterator for EphemerisVectorParser<'a, Input> {
    type Item = EphemerisVectorItem;

    fn next(&mut self) -> Option<Self::Item> {
        self.parser.next().map(|v| Self::Item::from(v))
    }
}

impl<'a, Input: Iterator<Item = &'a str>> Iterator for EphemerisOrbitalElementsParser<'a, Input> {
    type Item = EphemerisOrbitalElementsItem;

    fn next(&mut self) -> Option<Self::Item> {
        self.parser.next().map(|v| Self::Item::from(v))
    }
}
 */
#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;
    use crate::ephemeris::{EphemerisOrbitalElementsParser, EphemerisVectorParser};

    #[test]
    fn test_parsing_ephemeris_vector() {
        let data = include_str!("../vector.txt");
        let ephem: Vec<_> = EphemerisVectorParser::parse(data.lines())
            .map(|e| EphemerisVectorItem::from(e))
            .collect();
        assert_eq!(4, ephem.len());
        // TODO: This will probably fail intermittently due to float comparison.
        assert_eq!(
            EphemerisVectorItem {
                time: Utc.with_ymd_and_hms(2022, 8, 13, 19, 55, 56).unwrap(), // A.D. 2022-Aug-13 19:55:56.0000 TDB
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
        let ephem: Vec<_> = EphemerisOrbitalElementsParser::parse(data.lines())
            .map(|e| EphemerisOrbitalElementsItem::from(e))
            .collect();
        assert_eq!(4, ephem.len());
        // TODO: This will probably fail intermittently due to float comparison.
        assert_eq!(
            EphemerisOrbitalElementsItem {
                time: Utc.with_ymd_and_hms(2022, 6, 19, 18, 0, 0).unwrap(), // A.D. 2022-Jun-19 18:00:00.0000 TDB

                eccentricity: 1.711794334680415E-02,
                periapsis_distance: Length::new::<length::kilometer>(1.469885520304013E+08),
                inclination: Angle::new::<angle::degree>(3.134746902320420E-03),

                longitude_of_ascending_node: Angle::new::<angle::degree>(1.633896137466430E+02),
                argument_of_perifocus: Angle::new::<angle::degree>(3.006492364709574E+02),
                time_of_periapsis: Time::new::<time::day>(2459584.392523936927),

                mean_motion: AngularVelocity::new::<angular_velocity::degree_per_second>(
                    1.141316101270797E-05
                ),
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
