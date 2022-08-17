Access NASA JPL Horizons system from Rust.


Useful links
------------
- https://ssd.jpl.nasa.gov/horizons/
- https://ssd-api.jpl.nasa.gov/doc/horizons.html


Running the example
-------------------
The app doesn't have any command line options yet. To start seeing logs,
you can use `RUST_LOG` environment variable.

    RUST_LOG=trace cargo run --example major_bodies
