use chrono::{Duration, Utc};
use rhorizons::{
    ephemeris_orbital_elements_si, major_bodies, EphemerisOrbitalElementsItem, SiUnits,
};

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

    let elements: Vec<EphemerisOrbitalElementsItem<f32, SiUnits>> =
        ephemeris_orbital_elements_si(earth.id, start_time, stop_time).await;
    for item in elements {
        println!(
            "Eccentricity: {:?}, Semi-major axis: {:?}, Inclination: {:?}, Longitude of ascending node: {:?}, Argument of perifocus: {:?}, Mean anomaly: {:?}",
            item.eccentricity, item.semi_major_axis, item.inclination, item.longitude_of_ascending_node, item.argument_of_perifocus, item.mean_anomaly
        );
    }
}
