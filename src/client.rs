use std::time::Instant;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::parsing::MajorBody;

#[derive(Deserialize, Debug)]
struct HorizonsResponse {
    result: String,
}

/// Query the Horizons API, returning a result in form of lines.
async fn query<T>(parameters: &T) -> Vec<String>
where
    T: Serialize + ?Sized,
{
    let result = reqwest::Client::new()
        .get("https://ssd.jpl.nasa.gov/api/horizons.api")
        .query(parameters)
        .send()
        .await
        .unwrap()
        .json::<HorizonsResponse>()
        .await
        .unwrap()
        .result
        .split('\n')
        .map(str::to_owned)
        .collect::<Vec<String>>();

    for line in &result {
        log::trace!("{}", line);
    }

    result
}

pub async fn major_bodies() -> Vec<MajorBody> {
    query(&[("COMMAND", "MB")])
        .await
        .iter()
        .filter_map(|s| MajorBody::try_from(s.as_str()).ok())
        .collect()
}

pub async fn ephemeris(id: i32, start_time: DateTime<Utc>, stop_time: DateTime<Utc>) {
    query(&[
        ("COMMAND", id.to_string().as_str()),
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
}
