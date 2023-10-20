#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

mod client;
mod ephemeris;
mod major_bodies;
mod utilities;

#[cfg(feature = "si")]
/// Ephemeris information based on SI-units.
/// 
/// SI-units from the crate *uom*: <https://docs.rs/uom/0.35.0/uom/>
pub mod si;

pub use client::{ephemeris_orbital_elements, ephemeris_vector, major_bodies};
pub use ephemeris::{EphemerisOrbitalElementsItem, EphemerisVectorItem};
pub use major_bodies::MajorBody;
