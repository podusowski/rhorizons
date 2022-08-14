use chrono::Utc;
use client::major_bodies;

use crate::client::ephemeris;

mod client;
mod parsing;

#[tokio::main]
async fn main() {
    env_logger::init();
    //for body in major_bodies().await {
    //    eprintln!("{:?}", body);
    //}

    if let Some(body) = major_bodies()
        .await
        .iter()
        .find(|body| body.name == "Earth")
    {
        eprintln!("{:?}", body);
        ephemeris(body.id, Utc::now() - chrono::Duration::days(1), Utc::now()).await;
    }
}
