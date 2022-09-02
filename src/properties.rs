//! Parse geophysical properties of a body.

use crate::utilities::{take_expecting, take_or_empty};

struct Properties {
    mass: f32,
}

fn parse_properties<'a>(data: impl Iterator<Item = &'a str>) -> Properties {
    // GEOPHYSICAL PROPERTIES (revised May 9, 2022):
    //  Vol. Mean Radius (km)    = 6371.01+-0.02   Mass x10^24 (kg)= 5.97219+-0.0006
    for line in data {
        let (_, right) = take_or_empty(line, 45);
        if let Ok(multiplier) = take_expecting(right, "Mass x10^") {
            let (multiplier, line) = take_or_empty(multiplier, 2);
            let multiplier = multiplier.parse::<f32>().unwrap();
            if let Ok(line) = take_expecting(line, " (kg)= ") {
                let (mass, rest) = take_or_empty(line, 7);
                let mass = mass.parse::<f32>().unwrap();
                let mass = mass * 10_f32.powf(multiplier);
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
