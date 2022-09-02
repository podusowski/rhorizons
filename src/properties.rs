//! Parse geophysical properties of a body.

use crate::utilities::{take_expecting, take_or_empty};

struct Properties {
    mass: f32,
}

fn parse_properties<'a>(data: impl Iterator<Item = &'a str>) -> Properties {
    // GEOPHYSICAL PROPERTIES (revised May 9, 2022):
    //  Vol. Mean Radius (km)    = 6371.01+-0.02   Mass x10^24 (kg)= 5.97219+-0.0006
    for input in data {
        let (_, input) = take_or_empty(input, 45);
        if let Ok(multiplier) = take_expecting(input, "Mass x10^") {
            let (exponent, input) = take_or_empty(multiplier, 2);
            let exponent = exponent.parse::<f32>().unwrap();
            if let Ok(line) = take_expecting(input, " (kg)= ") {
                let (mantissa, _) = take_or_empty(line, 7);
                let mantissa = mantissa.parse::<f32>().unwrap();
                let mass = mantissa * 10_f32.powf(exponent);
                return Properties { mass };
            }
        }
    }
    Properties { mass: 0. }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_mass() {
        let data = include_str!("ephem2.txt");
        let properties = parse_properties(data.lines());
        assert_eq!(5.97219E24, properties.mass);
    }
}
