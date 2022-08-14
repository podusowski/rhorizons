use serde::Deserialize;

use crate::parsing::MajorBody;

#[derive(Deserialize, Debug)]
struct HorizonsResponse {
    result: String,
}

pub async fn major_bodies() -> Vec<MajorBody> {
    reqwest::Client::new()
        .get("https://ssd.jpl.nasa.gov/api/horizons.api")
        .query(&[("COMMAND", "MB")])
        .send()
        .await
        .unwrap()
        .json::<HorizonsResponse>()
        .await
        .unwrap()
        .result
        .split('\n')
        .filter_map(|s| MajorBody::try_from(s).ok())
        .collect()
}
