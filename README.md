[![crates.io](https://img.shields.io/crates/v/rhorizons.svg)](https://crates.io/crates/rhorizons)

Access NASA JPL Horizons system from Rust. This crate is written in asynchronous
code, therefore you probably want to use it in conjunction with `tokio`.

## Example

```rust
#[tokio::main]
async fn main() {
    println!("Major bodies in the Solar System.");

    for body in rhorizons::major_bodies().await {
        println!("{} ({})", body.name, body.id);
    }
}
```

You can check more examples in
[the source repository](https://github.com/podusowski/rhorizons/tree/main/examples).

## Useful links

- <https://ssd.jpl.nasa.gov/horizons/>
- <https://ssd-api.jpl.nasa.gov/doc/horizons.html>
