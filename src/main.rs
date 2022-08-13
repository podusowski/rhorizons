use std::{num::ParseIntError, str::FromStr};

use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize, Debug)]
struct HorizonsResponse {
    result: String,
}

#[tokio::main]
async fn main() {
    let response = reqwest::Client::new()
        .get("https://ssd.jpl.nasa.gov/api/horizons.api")
        .query(&[("COMMAND", "MB")])
        .send()
        .await
        .unwrap()
        .json::<HorizonsResponse>()
        .await
        .unwrap();

    eprintln!("{:?}", response);

    for line in response.result.split("\n") {
        eprintln!("{}", line);
        eprintln!("{:?}", MajorBody::try_from(line));
    }
}

/// Planet, natural satellite, spacecraft, Sun, barycenter, or other objects
/// having pre-computed trajectories.
///
/// https://ssd.jpl.nasa.gov/horizons/manual.html#defs
#[derive(Debug)]
struct MajorBody {
    id: i32,
}

#[derive(Error, Debug)]
enum MajorBodyParseError {
    #[error("invalid id")]
    InvalidId(#[source] ParseIntError),
}

impl TryFrom<&str> for MajorBody {
    type Error = MajorBodyParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            // TODO: Emit some meaningful error.
            id: value
                .get(0..9)
                .unwrap_or("none")
                .trim()
                .parse()
                .map_err(MajorBodyParseError::InvalidId)?,
        })
    }
}
