use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::si::ephemeris::{
        EphemerisOrbitalElementsItem, EphemerisOrbitalElementsParser, EphemerisVectorItem,
        EphemerisVectorParser,
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

/// Get vector ephemeris (position and velocity) of a major body. Coordinates are
/// relative to the Sun's center.
pub async fn ephemeris_vector(
    id: i32,
    start_time: DateTime<Utc>,
    stop_time: DateTime<Utc>,
) -> Vec<EphemerisVectorItem> {
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
) -> Vec<EphemerisOrbitalElementsItem> {
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
