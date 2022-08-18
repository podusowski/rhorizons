use rhorizons::major_bodies;

#[tokio::main]
async fn main() {
    env_logger::init();

    println!("Major bodies in the Solar System.");

    for body in major_bodies().await {
        println!("{:?}", body);
    }
}
