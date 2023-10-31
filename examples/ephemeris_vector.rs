use chrono::{Duration, Utc};
use rhorizons::{ephemeris_vector, major_bodies, DefaultUnits, EphemerisVectorItem};

#[tokio::main]
async fn main() {
    env_logger::init();

    let bodies = major_bodies().await;

    let earth = bodies
        .iter()
        .find(|body| body.name == "Earth")
        .expect("could not find Earth in Horizons");

    println!("Found Earth's Horizons identifier: {}.", earth.id);

    let start_time = Utc::now() - Duration::days(1);
    let stop_time = Utc::now();

    println!(
        "Earth's positions and velocities from {} to {}:",
        start_time, stop_time
    );

    let vectors: Vec<EphemerisVectorItem<f32, DefaultUnits>> =
        ephemeris_vector(earth.id, start_time, stop_time).await;

    for item in vectors {
        println!(
            "position: {:?}, velocity: {:?}",
            item.position, item.velocity
        );
    }
}
