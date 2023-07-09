/// Tests in this module connect to the real Horizons system. As such, they
/// require Internet access and might start failing if Horizon's API changes.
use chrono::{TimeZone, Utc};
use rhorizons::*;

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[tokio::test]
async fn finding_earth() {
    init();

    let bodies = major_bodies().await;
    let earth = bodies.iter().find(|body| body.name == "Earth").unwrap();

    assert_eq!(399, earth.id);
}

#[tokio::test]
async fn getting_earths_ephemeris() {
    init();

    // 2457677.000000000 = A.D. 2016-Oct-15 12:00:00.0000 TDB
    //  X = 1.379561021896053E+08 Y = 5.667156012930278E+07 Z =-2.601196352168918E+03
    //  VX=-1.180102398133564E+01 VY= 2.743089439727051E+01 VZ= 3.309367894566151E-05
    //  LT= 4.974865749957088E+02 RG= 1.491427231399648E+08 RR=-4.926267109444211E-01
    let vectors = ephemeris_vector(
        399,
        Utc.with_ymd_and_hms(2016, 10, 15, 12, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2016, 10, 15, 13, 0, 0).unwrap(),
    )
    .await;

    assert_eq!(1.379561021896053E+08, vectors[0].position[0]);
}

#[tokio::test]
async fn getting_jupiter_ephemeris() {
    init();

    // Target body name: Jupiter (599)                   {source: jup365_merged}
    // Center body name: Sun (10)                        {source: jup365_merged}
    // 2457677.000000000 = A.D. 2016-Oct-15 12:00:00.0000 TDB
    //  X =-8.125930353044792E+08 Y =-6.890018021386522E+07 Z = 1.846888215010012E+07
    //  VX= 9.479984730623543E-01 VY=-1.241342015681963E+01 VZ= 3.033885124560420E-02
    //  LT= 2.720942202383012E+03 RG= 8.157179509283365E+08 RR= 1.048282114626244E-01
    let vectors = ephemeris_vector(
        599,
        Utc.with_ymd_and_hms(2016, 10, 15, 12, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2016, 10, 15, 13, 0, 0).unwrap(),
    )
    .await;

    assert_eq!(-8.125930353044792E+08, vectors[0].position[0]);
}
