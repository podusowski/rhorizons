use chrono::{DateTime, Utc};

use crate::si::ephemeris::{EphemerisOrbitalElementsItem, EphemerisVectorItem};

/// Get vector ephemeris (position and velocity) of a major body. Coordinates are
/// relative to the Sun's center.
pub async fn ephemeris_vector(
    id: i32,
    start_time: DateTime<Utc>,
    stop_time: DateTime<Utc>,
) -> Vec<EphemerisVectorItem> {
    crate::ephemeris_vector(id, start_time, stop_time)
        .await
        .into_iter()
        .map(EphemerisVectorItem::from)
        .collect()
}
/// Get orbital element ephemeris (e.g. eccentricity, semi-major axis, ...) of a
/// major body relative to the Sun's center
pub async fn ephemeris_orbital_elements(
    id: i32,
    start_time: DateTime<Utc>,
    stop_time: DateTime<Utc>,
) -> Vec<EphemerisOrbitalElementsItem> {
    crate::ephemeris_orbital_elements(id, start_time, stop_time)
        .await
        .into_iter()
        .map(EphemerisOrbitalElementsItem::from)
        .collect()
}
