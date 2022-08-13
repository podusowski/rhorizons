use serde::Deserialize;

use crate::parsing::MajorBody;

mod parsing;

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
