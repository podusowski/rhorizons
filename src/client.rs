use serde::Deserialize;

use crate::parsing::MajorBody;

#[derive(Deserialize, Debug)]
struct HorizonsResponse {
    result: String,
}

/// Query the Horizons API, returning a result in form of lines.
async fn query() -> Vec<String> {
    let result = reqwest::Client::new()
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
        .map(str::to_owned)
        .collect::<Vec<String>>();

    for line in &result {
        log::trace!("{}", line);
    }

    result
}

pub async fn major_bodies() -> Vec<MajorBody> {
    query()
        .await
        .iter()
        .filter_map(|s| MajorBody::try_from(s.as_str()).ok())
        .collect()
}
