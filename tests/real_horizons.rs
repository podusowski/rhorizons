/// Tests in this module connect to the real Horizons system. As such, they
/// require Internet access and might start failing if Horizon's API changes.
use rhorizons::*;

#[tokio::test]
async fn finding_earth() {
    let bodies = major_bodies().await;
    let earth = bodies.iter().find(|body| body.name == "Earth").unwrap();

    assert_eq!(399, earth.id);
}
