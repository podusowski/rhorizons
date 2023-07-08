use chrono::{Duration, Utc};
use rhorizons::{ephemeris_orbital_elements, major_bodies};

#[tokio::main]
async fn main() {
    env_logger::init();

    let bodies = major_bodies().await;

    let earth = bodies
        .iter()
        .find(|body| body.name == "Mars")
        .expect("could not find Mars in Horizons");

    println!("Found Mars's Horizons identifier: {}.", earth.id);

    let start_time = Utc::now() - Duration::days(1);
    let stop_time = Utc::now();

    println!(
        "Mars's orbital elements from {} to {}:",
        start_time, stop_time
    );

    for elements in ephemeris_orbital_elements(earth.id, start_time, stop_time).await {
        println!(
            "Eccentricity: {:?}, Semi-major axis: {:?}, Inclination: {:?}, Longitude of ascending node: {:?}, Argument of perifocus: {:?}, Mean anomaly: {:?}",
            elements.eccentricity, elements.semi_major_axis, elements.inclination, elements.longitude_of_ascending_node, elements.argument_of_perifocus, elements.mean_anomaly
        );
    }
}
