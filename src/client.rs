use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    ephemeris::{
        EphemerisOrbitalElementsItem, EphemerisOrbitalElementsParser, EphemerisVectorItem,
        EphemerisVectorParser,
    },
    major_bodies::MajorBody,
};

/// Generic Horizons response. Their API just gives some JSON with two field,
/// some statuses and `result` field which is just human-readable string
/// normally seen in telnet or web API.
#[derive(Deserialize, Debug)]
struct HorizonsResponse {
    result: String,
}

#[derive(Error, Debug)]
#[error("error returned from Horizons")]
struct HorizonsQueryError;

/// Query the Horizons API, returning a result in form of lines.
async fn query<T>(parameters: &T) -> Result<Vec<String>, HorizonsQueryError>
where
    T: Serialize,
{
    let result = reqwest::Client::new()
        .get("https://ssd.jpl.nasa.gov/api/horizons.api")
        .query(parameters)
        .send()
        .await
        .map_err(|_| HorizonsQueryError)?
        .json::<HorizonsResponse>()
        .await
        .map_err(|_| HorizonsQueryError)?
        .result
        .split('\n')
        .map(str::to_owned)
        .collect::<Vec<String>>();

    for line in &result {
        log::trace!("{}", line);
    }

    Ok(result)
}

async fn query_with_retries<T>(parameters: &T) -> Vec<String>
where
    T: Serialize,
{
    for n in 1..10 {
        log::trace!("try {}", n);
        if let Ok(result) = query(parameters).await {
            return result;
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await
    }
    // TODO: Don't panic.
    panic!("max retries exceeded");
}

/// Get names and identifiers of all major bodies in the Solar System.
pub async fn major_bodies() -> Vec<MajorBody> {
    query_with_retries(&[("COMMAND", "MB")])
        .await
        .iter()
        .filter_map(|s| MajorBody::try_from(s.as_str()).ok())
        .collect()
}

/// Get vector ephemeris (position and velocity) of a major body. Coordinates are
/// relative to the Sun's center.
pub async fn ephemeris_vector(
    id: i32,
    start_time: DateTime<Utc>,
    stop_time: DateTime<Utc>,
) -> Vec<EphemerisVectorItem<f32, crate::units::DefaultUnits>> {
    let result = query_with_retries(&[
        ("COMMAND", id.to_string().as_str()),
        // Select Sun as a observer. Note that Solar System Barycenter is in a
        // slightly different place.
        // https://astronomy.stackexchange.com/questions/44851/
        ("CENTER", "500@10"),
        ("EPHEM_TYPE", "VECTORS"),
        // https://ssd.jpl.nasa.gov/horizons/manual.html#time
        (
            "START_TIME",
            start_time.format("%Y-%b-%d-%T").to_string().as_str(),
        ),
        (
            "STOP_TIME",
            stop_time.format("%Y-%b-%d-%T").to_string().as_str(),
        ),
    ])
    .await;

    EphemerisVectorParser::parse(result.iter().map(String::as_str)).collect()
}
/// Get orbital element ephemeris (e.g. eccentricity, semi-major axis, ...) of a
/// major body relative to the Sun's center
pub async fn ephemeris_orbital_elements(
    id: i32,
    start_time: DateTime<Utc>,
    stop_time: DateTime<Utc>,
) -> Vec<EphemerisOrbitalElementsItem<f32, crate::units::DefaultUnits>> {
    let result = query_with_retries(&[
        ("COMMAND", id.to_string().as_str()),
        // Select Sun as a observer. Note that Solar System Barycenter is in a
        // slightly different place.
        // https://astronomy.stackexchange.com/questions/44851/
        ("CENTER", "500@10"),
        ("EPHEM_TYPE", "ELEMENTS"),
        // https://ssd.jpl.nasa.gov/horizons/manual.html#time
        (
            "START_TIME",
            start_time.format("%Y-%b-%d-%T").to_string().as_str(),
        ),
        (
            "STOP_TIME",
            stop_time.format("%Y-%b-%d-%T").to_string().as_str(),
        ),
    ])
    .await;

    EphemerisOrbitalElementsParser::parse(result.iter().map(String::as_str)).collect()
}

#[cfg(feature = "si")]
/// Get vector ephemeris (position and velocity) of a major body in SI-units. Coordinates are
/// relative to the Sun's center.
/// Needs the `si` feature to be enabled
///
/// SI-units from the crate *uom*: <https://docs.rs/uom/0.35.0/uom/>
pub async fn ephemeris_vector_si(
    id: i32,
    start_time: DateTime<Utc>,
    stop_time: DateTime<Utc>,
) -> Vec<EphemerisVectorItem<f32, crate::units::SiUnits>> {
    crate::ephemeris_vector(id, start_time, stop_time)
        .await
        .into_iter()
        .map(EphemerisVectorItem::from)
        .collect()
}

#[cfg(feature = "si")]
/// Get orbital element ephemeris (e.g. eccentricity, semi-major axis, ...) of a
/// major body relative to the Sun's center in SI-units.
/// Needs the `si` feature to be enabled
///
/// SI-units from the crate *uom*: <https://docs.rs/uom/0.35.0/uom/>
pub async fn ephemeris_orbital_elements_si(
    id: i32,
    start_time: DateTime<Utc>,
    stop_time: DateTime<Utc>,
) -> Vec<EphemerisOrbitalElementsItem<f32, crate::units::SiUnits>> {
    crate::ephemeris_orbital_elements(id, start_time, stop_time)
        .await
        .into_iter()
        .map(EphemerisOrbitalElementsItem::from)
        .collect()
}
