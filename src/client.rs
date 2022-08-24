use std::future::Future;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    ephemeris::{EphemerisItem, EphemerisParser},
    major_bodies::MajorBody,
};

/// Generic Horizons response. Their API just gives some JSON with two field,
/// some statuses and `result` field which is just human-readable string
/// normally seen in telnet or web API.
#[derive(Deserialize, Debug)]
struct HorizonsResponse {
    result: String,
}

/// Opinionated retry for async functions. Actual number of retries and delay
/// between them is implementation detail and cannot be parametrized.
async fn retry_couple_times<F, R, E>(f: impl Fn() -> F) -> R
where
    F: Future<Output = Result<R, E>>,
{
    for n in 1..10 {
        log::trace!("try {}", n);
        if let Ok(result) = f().await {
            return result;
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await
    }
    // TODO: Don't panic.
    panic!("max retries exceeded");
}

#[derive(Error, Debug)]
#[error("error returned from Horizons")]
struct HorizonsQueryError;

/// Query the Horizons API, returning a result in form of lines.
async fn query<T>(parameters: &T) -> Vec<String>
where
    T: Serialize,
{
    retry_couple_times(async || -> Result<Vec<String>, HorizonsQueryError> {
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
    })
    .await
}

/// Get names and identifiers of all major bodies in the Solar System.
pub async fn major_bodies() -> Vec<MajorBody> {
    query(&[("COMMAND", "MB")])
        .await
        .iter()
        .filter_map(|s| MajorBody::try_from(s.as_str()).ok())
        .collect()
}

/// Get ephemeris (position and velocity) of a major body. Coordinates are
/// relative to the Sun's center.
pub async fn ephemeris(
    id: i32,
    start_time: DateTime<Utc>,
    stop_time: DateTime<Utc>,
) -> Vec<EphemerisItem> {
    let result = query(&[
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

    EphemerisParser::parse(result.iter().map(String::as_str)).collect()
}
