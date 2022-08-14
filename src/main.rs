use client::major_bodies;

mod client;
mod parsing;

#[tokio::main]
async fn main() {
    env_logger::init();
    for body in major_bodies().await {
        eprintln!("{:?}", body);
    }
}
