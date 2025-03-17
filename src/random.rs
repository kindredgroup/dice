use std::ops::RangeInclusive;
use statrs::distribution::{ContinuousCDF, Normal};
use tinyrand::Rand;

#[inline]
pub fn uniform(rand: &mut impl Rand) -> f64 {
    rand.next_u64() as f64 / u64::MAX as f64
}

#[inline]
pub fn gaussian_3_sigma(rand: &mut impl Rand) -> f64 {
    gaussian(rand, &Normal::new(0.5, 0.5 / 3.0).unwrap(), &(0.0..=1.0))
}

#[inline]
pub fn gaussian(rand: &mut impl Rand, normal: &Normal, range: &RangeInclusive<f64>) -> f64 {
    loop {
        let p = normal.inverse_cdf(uniform(rand));
        if range.contains(&p) {
            return p;
        }
    }
}