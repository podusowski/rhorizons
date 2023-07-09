[![crates.io](https://img.shields.io/crates/v/walkers.svg)](https://crates.io/crates/walkers)

Access NASA JPL Horizons system from Rust. This crate is written in asynchronous
code, therefore you probably want to use it in conjunction with `tokio`.


## Examples

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
