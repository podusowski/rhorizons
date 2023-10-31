use num_traits::Float;

pub trait Units<F: Float> {
    type Angle;
    type AngularVelocity;
    type Length;
    type Time;
    type Velocity;
}

#[derive(Debug, PartialEq)]
/// Used to define default floating units for Ephemeris Items.
///
/// ```rust
/// use rhorizons::*;
/// let vector: EphemerisVectorItem<f32, DefaultUnits>;
/// ```
pub struct DefaultUnits;

#[cfg(feature = "si")]
#[derive(Debug, PartialEq)]
/// Used to define SI-based floating units for Ephemeris Items.
/// Needs the `si` feature to be enabled.
///
/// ```rust
/// use rhorizons::*;
/// let orbital_elements: EphemerisOrbitalElementsItem<f32, SiUnits>;
/// ```
pub struct SiUnits;

impl<F: Float> Units<F> for DefaultUnits {
    type Angle = F;
    type AngularVelocity = F;
    type Length = F;
    type Time = F;
    type Velocity = F;
}

#[cfg(feature = "si")]
impl<F> Units<F> for SiUnits
where
    F: Float + uom::Conversion<F, T = F>,
    uom::si::length::meter: uom::Conversion<F, T = F>,
    uom::si::mass::kilogram: uom::Conversion<F, T = F>,
    uom::si::time::second: uom::Conversion<F, T = F>,
    uom::si::electric_current::ampere: uom::Conversion<F, T = F>,
    uom::si::thermodynamic_temperature::kelvin: uom::Conversion<F, T = F>,
    uom::si::amount_of_substance::mole: uom::Conversion<F, T = F>,
    uom::si::luminous_intensity::candela: uom::Conversion<F, T = F>,
{
    type Angle = uom::si::angle::Angle<uom::si::SI<F>, F>;
    type AngularVelocity = uom::si::angular_velocity::AngularVelocity<uom::si::SI<F>, F>;
    type Length = uom::si::length::Length<uom::si::SI<F>, F>;
    type Time = uom::si::time::Time<uom::si::SI<F>, F>;
    type Velocity = uom::si::velocity::Velocity<uom::si::SI<F>, F>;
}
