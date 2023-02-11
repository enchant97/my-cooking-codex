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

    pub fn to_f32(&self) -> f32 {
        self.numerator as f32 / self.denominator as f32
    }

    pub fn to_f64(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
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
