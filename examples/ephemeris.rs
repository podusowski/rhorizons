use chrono::Utc;
use rhorizons::{ephemeris, major_bodies};

#[tokio::main]
async fn main() {
    env_logger::init();

    let bodies = major_bodies().await;

    let earth = bodies
        .iter()
        .find(|body| body.name == "Earth")
        .expect("could not find Earth in Horizons");

    println!("Found Earth's Horizons identifier: {}.", earth.id);

    let start_time = Utc::now() - chrono::Duration::days(1);
    let stop_time = Utc::now();

    println!("Ephemeris:");

    for vectors in ephemeris(earth.id, start_time, stop_time).await {
        println!("{:?}", vectors);
    }
}
