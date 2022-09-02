use rhorizons::geophysical_properties;

#[tokio::main]
async fn main() {
    env_logger::init();

    println!("Earth's mass: {}kg", geophysical_properties(399).await.mass);
    //println!("Jupiter's mass: {}kg", geophysical_properties(599).await.mass);
    println!("Europa's mass: {}kg", geophysical_properties(502).await.mass);
}
