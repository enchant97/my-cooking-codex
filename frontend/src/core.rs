pub mod api;
pub mod effects;
pub mod storage;
pub mod types;

use std::str::FromStr;

/// A fraction type that can be converted to f32 and f64.
pub struct Fraction {
    pub numerator: i32,
    pub denominator: i32,
}

impl Fraction {
    pub fn new(numerator: i32, denominator: i32) -> Self {
        Self {
            numerator,
            denominator,
        }
    }
}

impl From<f32> for Fraction {
    fn from(f: f32) -> Self {
        // check if the float is already a whole number
        if f.fract() == 0.0 {
            return Self::new(f as i32, 1);
        }
        // otherwise, find the closest fraction
        // FIXME will not work for all cases
        let mut numerator = 1;
        let mut denominator = 1;
        while (numerator as f32 / denominator as f32 - f).abs() > 0.0001 {
            denominator += 1;
            numerator = (f * denominator as f32).round() as i32;
        }
        Self::new(numerator, denominator)
    }
}

impl FromStr for Fraction {
    type Err = ();

    /// Parse a fraction from a string.
    /// The string must be in the format of `numerator/denominator`.
    /// FIXME make this more robust (remove unwrap usage and return actual errors)
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split('/').collect();
        if parts.len() != 2 {
            return Err(());
        }
        let numerator = parts[0].parse().unwrap();
        let denominator = parts[1].parse().unwrap();
        Ok(Fraction {
            numerator,
            denominator,
        })
    }
}

impl Into<f32> for Fraction {
    fn into(self) -> f32 {
        self.numerator as f32 / self.denominator as f32
    }
}

impl std::fmt::Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // if the fraction is a whole number, just return the numerator
        if self.denominator == 1 {
            return write!(f, "{}", self.numerator);
        }
        // otherwise, return the fraction
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}
