//! Utilities for working with probabilities.

use crate::matrix::Matrix;
use std::fmt::{Display, Formatter};
use std::iter::Map;
use std::slice::Iter;
use tinyrand::Rand;

pub trait SliceExt {
    fn sum(&self) -> f64;
    fn normalise(&mut self, target: f64) -> f64;
    fn invert(&self) -> Map<Iter<f64>, fn(&f64) -> f64>;
    fn geometric_mean(&self) -> f64;
    fn dilate_additive(&mut self, factor: f64);
    fn dilate_power(&mut self, factor: f64);
    fn scale(&mut self, factor: f64);
    fn scale_rows(&self, target: &mut Matrix<f64>);
    fn dilate_rows_additive(&self, matrix: &mut Matrix<f64>);
    fn dilate_rows_power(&self, matrix: &mut Matrix<f64>);
    fn mean(&self) -> f64;
    fn variance(&self) -> f64;
    fn stdev(&self) -> f64;
    fn booksum(&self) -> f64;
    fn fill_random_probs<R: Rand, D: Fn(&mut R) -> f64>(&mut self, rand: &mut R, distribution: &D, sum: f64);
    
    /// Fills the slice with randomly assigned probabilities skewed to favour
    /// decreasing order (higher probabilities more likely appearing in the beginning). 
    /// 
    /// The algorithm works in two passes. The first pass assigns random probabilities from a
    /// diminishing probability space `remaining`, initialised with 1. For each element in 
    /// the `self` slice, values are sampled from a uniform distribution in the range constrained 
    /// by `remaining * (self.len)^-beta`. Thereafter, the sampled value is subtracted from
    /// `remaining`. The second pass simply scales the resulting probabilities 
    /// to sum to `normal`.
    /// 
    /// E.g., for a 25-element slice and `beta` = 0.5, the highest admissible probability 
    /// will be 0.2 (1/25^0.5) before normalisation. 
    /// 
    /// Over an infinite number of trials, the probabilities converge on an exponential that can 
    /// be varied with the scale parameter `beta`.
    fn fill_random_probs_exp<R: Rand, D: Fn(&mut R) -> f64>(&mut self, rand: &mut R, distribution: &D, beta: f64, sum: f64);

    /// Total sum of squares.
    fn sst(&self) -> f64;
    
    fn min(&self) -> f64;
    
    fn max(&self) -> f64;
    
    /// Iteratively redistributes probabilities so that no element exceeds 1.0 and any excess
    /// is uniformly distributed among the remaining elements.
    fn redistribute(&mut self);
}
impl SliceExt for [f64] {
    #[inline]
    fn sum(&self) -> f64 {
        self.iter().sum()
    }

    #[inline]
    fn normalise(&mut self, target: f64) -> f64 {
        let sum = self.sum();
        self.scale(target / sum);
        sum
    }

    #[inline]
    fn invert(&self) -> Map<Iter<f64>, fn(&f64) -> f64> {
        self.iter().map(|value| 1.0 / value)
    }

    #[inline]
    fn geometric_mean(&self) -> f64 {
        let product: f64 = self.iter().product();
        product.powf(1.0 / self.len() as f64)
    }

    #[inline]
    fn dilate_additive(&mut self, factor: f64) {
        #[inline(always)]
        fn dilate_additive_pve(slice: &mut [f64], factor: f64) {
            let share = factor / slice.len() as f64;
            for element in slice {
                *element = (*element + share) / (1.0 + factor);
            }
        }

        #[inline(always)]
        fn dilate_additive(slice: &mut [f64], factor: f64) {
            let share = factor / slice.len() as f64;
            let mut sum = 0.0;
            for element in &mut *slice {
                *element = f64::max(0.0, *element + share);
                sum += *element;
            }
            slice.scale(1.0 / sum);
        }

        if factor >= 0.0 {
            dilate_additive_pve(self, factor);
        } else {
            dilate_additive(self, factor);
        }
    }

    #[inline]
    fn dilate_power(&mut self, factor: f64) {
        let mut sum = 0.0;
        for element in &mut *self {
            *element = element.powf(1.0 - factor);
            sum += *element;
        }
        self.scale(1.0 / sum);
    }

    #[inline]
    fn scale(&mut self, factor: f64) {
        for element in self {
            *element *= factor;
        }
    }

    #[inline]
    fn scale_rows(&self, target: &mut Matrix<f64>) {
        debug_assert_eq!(
            target.rows(),
            self.len(),
            "number of factors {} does not match number of rows {}",
            self.len(),
            target.rows()
        );
        for (row, &factor) in self.iter().enumerate() {
            let row_slice = target.row_slice_mut(row);
            row_slice.scale(factor);
        }
    }

    #[inline]
    fn dilate_rows_additive(&self, matrix: &mut Matrix<f64>) {
        debug_assert_eq!(
            self.len(),
            matrix.rows(),
            "number of dilation factors {} must match the number of matrix rows {}",
            self.len(),
            matrix.rows()
        );
        for (row, factor) in self.iter().enumerate() {
            let row_slice = matrix.row_slice_mut(row);
            row_slice.dilate_additive(*factor);
        }
    }

    #[inline]
    fn dilate_rows_power(&self, matrix: &mut Matrix<f64>) {
        debug_assert_eq!(
            self.len(),
            matrix.rows(),
            "number of dilation factors {} must match the number of matrix rows {}",
            self.len(),
            matrix.rows()
        );
        for (row, factor) in self.iter().enumerate() {
            let row_slice = matrix.row_slice_mut(row);
            row_slice.dilate_power(*factor);
        }
    }

    #[inline]
    fn mean(&self) -> f64 {
        self.sum() / self.len() as f64
    }

    #[inline]
    fn variance(&self) -> f64 {
        let mean = self.mean();
        let sum_of_squares: f64 = self.iter().map(|sample| (sample - mean).powi(2)).sum();
        sum_of_squares / (self.len() - 1) as f64
    }

    #[inline]
    fn stdev(&self) -> f64 {
        self.variance().sqrt()
    }

    #[inline]
    fn booksum(&self) -> f64 {
        self.invert().sum()
    }

    #[inline]
    fn fill_random_probs<R: Rand, D: Fn(&mut R) -> f64>(&mut self, rand: &mut R, distribution: &D, sum: f64) {
        for prob in self.iter_mut() {
            *prob = distribution(rand);
        }
        self.normalise(sum);
    }

    #[inline]
    fn fill_random_probs_exp<R: Rand, D: Fn(&mut R) -> f64>(&mut self, rand: &mut R, distribution: &D, beta: f64, sum: f64) {
        let mut remaining = 1.0;
        let len = self.len();
        let scale = 1.0 / (len as f64).powf(beta);
        for prob in self.iter_mut() {
            *prob = distribution(rand) * remaining * scale;
            remaining -= *prob;
        }
        self.normalise(sum);
    }

    #[inline]
    fn sst(&self) -> f64 {
        let mean = self.mean();
        self.iter().map(|value| (mean - value).powi(2)).sum()
    }

    #[inline]
    fn min(&self) -> f64 {
        self.iter().map(|item| *item).reduce(|a, b| a.min(b)).unwrap()
    }

    #[inline]
    fn max(&self) -> f64 {
        self.iter().map(|item| *item).reduce(|a, b| a.max(b)).unwrap()
    }

    #[inline]
    fn redistribute(&mut self) {
        let orig_sum = self.sum();
        let max = self.max();
        if max > 1.0 {
            self.scale(1.0 / max);
            let mut sum = orig_sum / max;
            let mut capped_count = 1;
            loop {
                if sum >= orig_sum || capped_count == self.len() {
                    break;
                }
                let scale = (orig_sum - capped_count as f64) / (sum - capped_count as f64);
                sum = 0.0;
                capped_count = 0;
                for p in &mut self.iter_mut() {
                    *p = f64::min(*p * scale, 1.0);
                    sum += *p;
                    if *p >= 1.0 {
                        capped_count += 1;
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fraction {
    pub numerator: u64,
    pub denominator: u64,
}
impl Fraction {
    pub fn quotient(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix::matrix_fixtures::populate_with_test_data;
    use crate::testing::{assert_slice_f64_near, assert_slice_f64_relative};
    use assert_float_eq::*;
    use tinyrand::StdRand;
    use crate::random;

    #[test]
    fn sum() {
        let data = [0.0, 0.1, 0.2];
        assert_f64_near!(0.3, data.sum(), 1);
    }

    #[test]
    fn mean() {
        let data = [0.05, 0.1, 0.15, 0.2];
        assert_f64_near!(0.125, data.mean());
    }

    #[test]
    fn variance() {
        let data = [0.05, 0.1, 0.15, 0.2];
        assert_float_relative_eq!(0.00416667, data.variance(), 1e-6);
    }

    #[test]
    fn normalise() {
        let mut data = [0.05, 0.1, 0.15, 0.2];
        let sum = data.normalise(1.0);
        assert_f64_near!(0.5, sum, 1);
        assert_slice_f64_near(&[0.1, 0.2, 0.3, 0.4], &data, 1);
    }

    #[test]
    fn scale_rows() {
        let mut matrix = Matrix::allocate(3, 2);
        populate_with_test_data(&mut matrix);
        [2.0, 4.0, 6.0].scale_rows(&mut matrix);
        assert_eq!(&[0.0, 20.0, 80.0, 120.0, 240.0, 300.0], matrix.flatten());
    }

    #[test]
    fn geometric_mean() {
        let data = [1.0, 3.0, 9.0];
        assert_f64_near!(3.0, data.geometric_mean());
    }

    #[test]
    fn dilate_additive_zero() {
        let mut data = [0.1, 0.2, 0.3, 0.4];
        data.dilate_additive(0.0);
        assert_slice_f64_near(&[0.1, 0.2, 0.3, 0.4], &data, 1);
    }

    #[test]
    fn dilate_additive_pve() {
        let mut data = [0.1, 0.2, 0.3, 0.4];
        data.dilate_additive(0.2);
        assert_slice_f64_relative(&[0.125, 0.2083, 0.2917, 0.375], &data, 0.0005);
    }

    #[test]
    fn dilate_additive_nve() {
        let mut data = [0.1, 0.2, 0.3, 0.4];
        data.dilate_additive(-0.2);
        assert_slice_f64_relative(&[0.0625, 0.1875, 0.3125, 0.4375], &data, 0.0005);
    }

    #[test]
    fn dilate_power_zero() {
        let mut data = [0.1, 0.2, 0.3, 0.4];
        data.dilate_power(0.0);
        assert_slice_f64_near(&[0.1, 0.2, 0.3, 0.4], &data, 1);
    }

    #[test]
    fn dilate_power_pve() {
        let mut data = [0.1, 0.2, 0.3, 0.4];
        data.dilate_power(0.2);
        assert_slice_f64_relative(&[0.1222, 0.2128, 0.2944, 0.3706], &data, 0.0005);
    }

    #[test]
    fn dilate_power_nve() {
        let mut data = [0.1, 0.2, 0.3, 0.4];
        data.dilate_power(-0.2);
        assert_slice_f64_relative(&[0.0812, 0.1866, 0.3035, 0.4287], &data, 0.0005);
    }
    
    #[test]
    fn booksum() {
        let data = [10.0, 5.0, 10.0/3.0, 2.5];
        let booksum = data.booksum();
        assert_f64_near!(1.0, booksum);
    }
    
    #[test]
    fn fill_random_probs() {
        let mut data = (0..20).map(|_| 0.0).collect::<Vec<_>>();
        data.fill_random_probs(&mut StdRand::default(), &random::uniform, 1.0);
        assert_f64_near!(1.0, data.sum());
    }

    #[test]
    fn fill_random_probs_exp() {
        let mut data = (0..20).map(|_| 0.0).collect::<Vec<_>>();
        data.fill_random_probs(&mut StdRand::default(), &random::uniform, 1.0);
        assert_f64_near!(1.0, data.sum());
    }

    #[test]
    fn sst() {
        let data = [0.1, 0.2, 0.3, 0.4];
        // expect sst = (.1-.25)^2+(.2-.25)^2+(.3-.25)^2+(.4-.25)^2
        assert_f64_near!(0.05, data.sst());
    }

    #[test]
    fn redistribute_no_caps() {
        let mut data = [0.95, 0.95, 0.85, 0.55, 0.5];
        data.redistribute();
        println!("data={data:?}");
        assert_slice_f64_relative(&[0.95, 0.95, 0.85, 0.55, 0.5], &data, 1e-9);
    }

    #[test]
    fn redistribute_one_caps() {
        let mut data = [1.1, 0.9, 0.85, 0.65, 0.5];
        data.redistribute();
        println!("data={data:?}");
        assert_slice_f64_relative(&[1.0, 0.9310344827586207, 0.8793103448275862, 0.6724137931034483, 0.5172413793103449], &data, 1e-9);
    }

    #[test]
    fn redistribute_two_caps() {
        let mut data = [1.2, 0.95, 0.85, 0.55, 0.45];
        data.redistribute();
        println!("data={data:?}");
        assert_slice_f64_relative(&[1.0, 1.0, 0.9189189189189191, 0.5945945945945946, 0.4864864864864865], &data, 1e-9);
    }
    
    #[test]
    fn redistribute_all_caps() {
        let mut data = [1.2706440060157531, 1.1021136420450872, 1.2632031482652073, 1.2052018371866131, 1.0635666775694972, 0.9003750771632325, 0.8717737136650149, 0.32312189808959524];
        data.redistribute();
        println!("data={data:?}");
        assert_slice_f64_relative(&[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0], &data, 1e-9);
    }
}
