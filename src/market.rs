use std::ops::RangeInclusive;

use crate::opt;
use crate::opt::UnivariateDescentConfig;
use crate::probs::SliceExt;

pub type PriceBounds = RangeInclusive<f64>;

pub trait MarketPrice {
    fn decimal(&self) -> f64;
}

impl MarketPrice for f64 {
    fn decimal(&self) -> f64 {
        *self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Overround {
    pub method: OverroundMethod,
    pub value: f64,
}
impl Overround {
    pub fn validate(&self) {
        const MIN_OVERROUND: f64 = 1.;
        if self.value < MIN_OVERROUND {
            panic!("overround cannot be less than {MIN_OVERROUND}");
        }
    }

    pub fn fair() -> Self {
        Self {
            method: OverroundMethod::Multiplicative,
            value: 1.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum OverroundMethod {
    Multiplicative,
    Power,
    OddsRatio,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Market {
    pub probs: Vec<f64>,
    pub prices: Vec<f64>,
    pub overround: Overround,
}
impl Market {
    pub fn validate(&self) {
        const VALID_PROB_RANGE: RangeInclusive<f64> = 0.0..=1.;
        if self
            .probs
            .iter()
            .any(|prob| !VALID_PROB_RANGE.contains(prob))
        {
            panic!("probabilities must lie in the range: {VALID_PROB_RANGE:?}");
        }
        const MIN_PRICE: f64 = 1.;
        if self.prices.iter().any(|&price| price < 1.) {
            panic!("prices cannot be lower than {MIN_PRICE}");
        }
        if self.prices.iter().any(|&price| price.is_nan()) {
            panic!("prices cannot be NaN");
        }
        const MIN_PROBS: usize = 1;
        if self.probs.len() < MIN_PROBS {
            panic!("the number of provided probabilities cannot be fewer than {MIN_PROBS}");
        }
        if self.probs.len() != self.prices.len() {
            panic!("exactly one probability must be provided for each price");
        }
        if self
            .probs
            .iter()
            .zip(self.prices.iter())
            .any(|(&prob, &price)| {
                prob == 0. && price.is_finite() || prob != 0. && price.is_infinite()
            })
        {
            panic!("a zero probability must be accompanied by an infinite price and vice versa");
        }
        self.overround.validate();
    }

    pub fn fair_booksum(&self) -> f64 {
        self.probs.sum()
    }

    pub fn offered_booksum(&self) -> f64 {
        self.prices.invert().sum()
    }

    pub fn fit(method: &OverroundMethod, prices: Vec<f64>, fair_sum: f64) -> Self {
        match method {
            OverroundMethod::Multiplicative => Self::fit_multiplicative(prices, fair_sum),
            OverroundMethod::Power => Self::fit_power(prices, fair_sum),
            OverroundMethod::OddsRatio => Self::fit_odds_ratio(prices, fair_sum),
        }
    }

    pub fn frame(overround: &Overround, probs: Vec<f64>, bounds: &PriceBounds) -> Self {
        match overround.method {
            OverroundMethod::Multiplicative => Self::frame_multiplicative(probs, overround.value, bounds),
            OverroundMethod::Power => Self::frame_power(probs, overround.value, bounds),
            OverroundMethod::OddsRatio => Self::frame_odds_ratio(probs, overround.value, bounds)
        }
    }

    fn fit_multiplicative(prices: Vec<f64>, fair_sum: f64) -> Self {
        let mut probs: Vec<_> = prices.invert().collect();
        let overround = probs.normalise(fair_sum) / fair_sum;
        Self {
            probs,
            prices,
            overround: Overround {
                method: OverroundMethod::Multiplicative,
                value: overround,
            },
        }
    }

    fn fit_power(prices: Vec<f64>, fair_sum: f64) -> Market {
        let overround = prices.invert().sum::<f64>() / fair_sum;
        let est_rtp = 1.0 / overround;
        let initial_k = 1.0 + f64::ln(est_rtp) / f64::ln(prices.len() as f64);
        let outcome = opt::univariate_descent(
            &UnivariateDescentConfig {
                init_value: initial_k,
                init_step: -0.01,
                min_step: 0.0001,
                max_steps: 100_000,
                acceptable_residual: 1e-9,
            },
            |exponent| {
                let mut sum = 0.0;
                for &price in &prices {
                    let scaled_price = (price * fair_sum).powf(exponent);
                    sum += 1.0 / scaled_price;
                }

                (sum - 1.0).powi(2)
            },
        );

        let probs = prices
            .iter()
            .map(|price| {
                let scaled_price = (price * fair_sum).powf(outcome.optimal_value);
                fair_sum / scaled_price
            })
            .collect();

        Self {
            probs,
            prices,
            overround: Overround {
                method: OverroundMethod::Power,
                value: overround,
            },
        }
    }

    fn fit_odds_ratio(prices: Vec<f64>, fair_sum: f64) -> Market {
        let overround = prices.invert().sum::<f64>() / fair_sum;
        let initial_d = overround;
        let outcome = opt::univariate_descent(
            &UnivariateDescentConfig {
                init_value: initial_d,
                init_step: 0.1,
                min_step: 0.0001,
                max_steps: 100_000,
                acceptable_residual: 1e-9,
            },
            |d| {
                let mut sum = 0.0;
                for &price in &prices {
                    let uncapped_scaled_price = 1.0 + (price - 1.0) / d;
                    sum += 1.0 / uncapped_scaled_price;
                }

                (sum - fair_sum).powi(2)
            },
        );

        let probs = prices
            .iter()
            .map(|price| {
                let scaled_price = 1.0 + (price - 1.0) / outcome.optimal_value;
                1.0 / scaled_price
            })
            .collect();

        Self {
            probs,
            prices,
            overround: Overround {
                method: OverroundMethod::OddsRatio,
                value: overround,
            },
        }
    }

    fn frame_multiplicative(probs: Vec<f64>, overround: f64, bounds: &PriceBounds) -> Self {
        let prices: Vec<_> = probs
            .iter()
            .map(|prob| multiply_capped(1.0 / prob, overround, bounds))
            .collect();
        Self {
            probs,
            prices,
            overround: Overround {
                method: OverroundMethod::Multiplicative,
                value: overround,
            },
        }
    }

    fn frame_power(probs: Vec<f64>, overround: f64, bounds: &PriceBounds) -> Market {
        let rtp = 1.0 / overround;
        let fair_sum = probs.sum();
        let initial_k = 1.0 + f64::ln(rtp) / f64::ln(probs.len() as f64);
        let min_scaled_price = 1.0 + (bounds.start() - 1.0) / fair_sum;
        let max_scaled_price = 1.0 + (bounds.end() - 1.0) / fair_sum;
        let outcome = opt::univariate_descent(
            &UnivariateDescentConfig {
                init_value: initial_k,
                init_step: -0.01,
                min_step: 0.0001,
                max_steps: 100_000,
                acceptable_residual: 1e-9,
            },
            |exponent| {
                let mut sum = 0.0;
                for &prob in &probs {
                    let uncapped_scaled_price = (fair_sum / prob).powf(exponent);
                    let capped_scaled_price =
                        cap(uncapped_scaled_price, min_scaled_price, max_scaled_price);
                    sum += 1.0 / capped_scaled_price;
                }

                (sum - overround).powi(2)
            },
        );

        let prices = probs
            .iter()
            .map(|prob| {
                let uncapped_price = (fair_sum / prob).powf(outcome.optimal_value) / fair_sum;
                if uncapped_price.is_finite() {
                    cap(uncapped_price, *bounds.start(), *bounds.end())
                } else {
                    uncapped_price
                }
            })
            .collect();

        Self {
            probs,
            prices,
            overround: Overround {
                method: OverroundMethod::Power,
                value: overround,
            },
        }
    }

    fn frame_odds_ratio(probs: Vec<f64>, overround: f64, bounds: &PriceBounds) -> Market {
        let fair_sum = probs.sum();
        let overround_sum = fair_sum * overround;
        let initial_d = overround;
        let outcome = opt::univariate_descent(
            &UnivariateDescentConfig {
                init_value: initial_d,
                init_step: 0.1,
                min_step: 0.0001,
                max_steps: 100_000,
                acceptable_residual: 1e-9,
            },
            |d| {
                let mut sum = 0.0;
                for &prob in &probs {
                    let price = 1.0 / prob;
                    let uncapped_scaled_price = 1.0 + (price - 1.0) / d;
                    let capped_scaled_price =
                        cap(uncapped_scaled_price, *bounds.start(), *bounds.end());
                    sum += 1.0 / capped_scaled_price;
                }

                (sum - overround_sum).powi(2)
            },
        );

        let prices = probs
            .iter()
            .map(|prob| {
                let price = 1.0 / prob;
                let uncapped_price = 1.0 + (price - 1.0) / outcome.optimal_value;
                if uncapped_price.is_finite() {
                    cap(uncapped_price, *bounds.start(), *bounds.end())
                } else {
                    uncapped_price
                }
            })
            .collect();

        Self {
            probs,
            prices,
            overround: Overround {
                method: OverroundMethod::OddsRatio,
                value: overround,
            },
        }
    }
}

#[inline]
pub fn multiply_capped(fair_price: f64, overround: f64, bounds: &PriceBounds) -> f64 {
    let quotient = fair_price / overround;
    if quotient.is_finite() {
        cap(quotient, *bounds.start(), *bounds.end())
    } else {
        quotient
    }
}

#[inline]
fn cap(value: f64, min: f64, max: f64) -> f64 {
    f64::min(f64::max(min, value), max)
}

#[cfg(test)]
mod tests {
    use assert_float_eq::*;
    use crate::testing::assert_slice_f64_relative;
    use super::*;

    const BOUNDS: PriceBounds = 1.04..=10_001.0;

    #[test]
    fn fit_multiplicative() {
        {
            let prices = vec![10.0, 5.0, 3.333, 2.5];
            let market = Market::fit(&OverroundMethod::Multiplicative, prices, 1.0);
            assert_slice_f64_relative(&[0.1, 0.2, 0.3, 0.4], &market.probs, 0.001);
            assert_float_absolute_eq!(1.0, market.overround.value, 0.001);
        }
        {
            let prices = vec![9.0909, 4.5454, 3.0303, 2.273];
            let market = Market::fit(&OverroundMethod::Multiplicative, prices, 1.0);
            println!("market: {:?}", market);
            assert_slice_f64_relative(&[0.1, 0.2, 0.3, 0.4], &market.probs, 0.001);
            assert_float_absolute_eq!(1.1, market.overround.value, 0.001);
        }
        {
            let prices = vec![9.0909, 4.5454, 3.0303, 2.273, f64::INFINITY];
            let market = Market::fit(&OverroundMethod::Multiplicative, prices, 1.0);
            println!("market: {:?}", market);
            assert_slice_f64_relative(&[0.1, 0.2, 0.3, 0.4, 0.0], &market.probs, 0.001);
            assert_float_absolute_eq!(1.1, market.overround.value, 0.001);
        }
        {
            let prices = vec![4.5454, 2.2727, 1.5152, 1.1364];
            let market = Market::fit(&OverroundMethod::Multiplicative, prices, 2.0);
            println!("market: {:?}", market);
            assert_slice_f64_relative(&[0.2, 0.4, 0.6, 0.8], &market.probs, 0.001);
            assert_float_absolute_eq!(1.1, market.overround.value, 0.001);
        }
        {
            let prices = vec![
                23.,
                6.5,
                8.,
                10.,
                5.5,
                11.,
                13.,
                3.7,
                27.,
                251.,
                16.,
                91.,
                126.,
                8.5,
                126.,
                201.,
                f64::INFINITY,
                f64::INFINITY,
            ];
            let market = Market::fit(&OverroundMethod::Multiplicative, prices, 1.0);
            println!("market: {:?}", market);
            assert_slice_f64_relative(
                &[
                    0.03356745745810524,
                    0.11877715715944932,
                    0.09650644019205257,
                    0.07720515215364206,
                    0.14037300391571284,
                    0.07018650195785642,
                    0.05938857857972466,
                    0.20866257338822172,
                    0.028594500797645205,
                    0.0030759024762407193,
                    0.048253220096026284,
                    0.00848408265424638,
                    0.006127393028066829,
                    0.09082959076899065,
                    0.006127393028066829,
                    0.0038410523459523407,
                    0.0,
                    0.0,
                ],
                &market.probs,
                0.001,
            );
            assert_float_absolute_eq!(1.29525, market.overround.value, 0.001);
        }
    }

    #[test]
    fn fit_power() {
        {
            let prices = vec![10.0, 5.0, 3.333, 2.5];
            let market = Market::fit(&OverroundMethod::Power, prices, 1.0);
            assert_slice_f64_relative(&[0.1, 0.2, 0.3, 0.4], &market.probs, 0.001);
            assert_float_absolute_eq!(1.0, market.overround.value, 0.001);
        }
        {
            let prices = vec![8.4319, 4.4381, 3.0489, 2.3359];
            let market = Market::fit(&OverroundMethod::Power, prices, 1.0);
            println!("market: {:?}", market);
            assert_slice_f64_relative(&[0.1, 0.2, 0.3, 0.4], &market.probs, 0.001);
            assert_float_absolute_eq!(1.1, market.overround.value, 0.001);
        }
        {
            let prices = vec![8.4319, 4.4381, 3.0489, 2.3359, f64::INFINITY];
            let market = Market::fit(&OverroundMethod::Power, prices, 1.0);
            println!("market: {:?}", market);
            assert_slice_f64_relative(&[0.1, 0.2, 0.3, 0.4, 0.0], &market.probs, 0.001);
            assert_float_absolute_eq!(1.1, market.overround.value, 0.001);
        }
        {
            let prices = vec![4.2159, 2.219, 1.5244, 1.168];
            let market = Market::fit(&OverroundMethod::Power, prices, 2.0);
            println!("market: {:?}", market);
            assert_slice_f64_relative(&[0.2, 0.4, 0.6, 0.8], &market.probs, 0.001);
            assert_float_absolute_eq!(1.1, market.overround.value, 0.001);
        }
    }

    #[test]
    fn fit_odds_ratio() {
        {
            let prices = vec![10.0, 5.0, 3.333, 2.5];
            let market = Market::fit(&OverroundMethod::OddsRatio, prices, 1.0);
            assert_slice_f64_relative(&[0.1, 0.2, 0.3, 0.4], &market.probs, 0.001);
            assert_float_absolute_eq!(1.0, market.overround.value, 0.001);
        }
        {
            let prices = vec![8.8335, 4.4816, 3.0309, 2.3056];
            let market = Market::fit(&OverroundMethod::OddsRatio, prices, 1.0);
            println!("market: {:?}", market);
            assert_slice_f64_relative(&[0.1, 0.2, 0.3, 0.4], &market.probs, 0.001);
            assert_float_absolute_eq!(1.1, market.overround.value, 0.001);
        }
        {
            let prices = vec![8.8335, 4.4816, 3.0309, 2.3056, f64::INFINITY];
            let market = Market::fit(&OverroundMethod::OddsRatio, prices, 1.0);
            println!("market: {:?}", market);
            assert_slice_f64_relative(&[0.1, 0.2, 0.3, 0.4, 0.0], &market.probs, 0.001);
            assert_float_absolute_eq!(1.1, market.overround.value, 0.001);
        }
        {
            let prices = vec![4.1132, 2.1675, 1.5189, 1.1946];
            let market = Market::fit(&OverroundMethod::OddsRatio, prices, 2.0);
            println!("market: {:?}", market);
            assert_slice_f64_relative(&[0.2, 0.4, 0.6, 0.8], &market.probs, 0.001);
            assert_float_absolute_eq!(1.1, market.overround.value, 0.001);
        }
        {
            let prices = vec![1.2494, 1.1109, 1.0647, 1.0416, f64::INFINITY];
            let market = Market::fit(&OverroundMethod::OddsRatio, prices, 1.0);
            println!("market: {:?}", market);
            assert_slice_f64_relative(&[0.1, 0.2, 0.3, 0.4, 0.0], &market.probs, 0.005);
            assert_float_absolute_eq!(3.6, market.overround.value, 0.001);
        }
    }

    #[test]
    fn frame_fair() {
        let probs = vec![0.1, 0.2, 0.3, 0.4];
        let market = Market::frame(&Overround::fair(),
                                   probs,
                                   &BOUNDS
        );
        assert_slice_f64_relative(&[10.0, 5.0, 3.333, 2.5], &market.prices, 0.001);
    }

    #[test]
    fn frame_multiplicative() {
        {
            let probs = vec![0.1, 0.2, 0.3, 0.4];
            let market = Market::frame(
                &Overround {
                    method: OverroundMethod::Multiplicative,
                    value: 1.0,
                },
                probs,
                &BOUNDS
            );
            assert_slice_f64_relative(&[10.0, 5.0, 3.333, 2.5], &market.prices, 0.001);
        }
        {
            let probs = vec![0.1, 0.2, 0.3, 0.4];
            let market = Market::frame(
                &Overround {
                    method: OverroundMethod::Multiplicative,
                    value: 1.1,
                },
                probs,
                &BOUNDS
            );
            assert_slice_f64_relative(&[9.0909, 4.5454, 3.0303, 2.273], &market.prices, 0.001);
        }
        {
            let probs = vec![0.1, 0.2, 0.3, 0.4, 0.0];
            let market = Market::frame(
                &Overround {
                    method: OverroundMethod::Multiplicative,
                    value: 1.1,
                },
                probs,
                &BOUNDS
            );
            assert_slice_f64_relative(
                &[9.0909, 4.5454, 3.0303, 2.273, f64::INFINITY],
                &market.prices,
                0.001,
            );
        }
        {
            let probs = vec![0.2, 0.4, 0.6, 0.8];
            let market = Market::frame(
                &Overround {
                    method: OverroundMethod::Multiplicative,
                    value: 1.1,
                },
                probs,
                &BOUNDS
            );
            assert_slice_f64_relative(&[4.5454, 2.2727, 1.5152, 1.1364], &market.prices, 0.001);
        }
    }

    #[test]
    fn frame_power() {
        {
            let probs = vec![0.1, 0.2, 0.3, 0.4];
            let market = Market::frame(
                &Overround {
                    method: OverroundMethod::Power,
                    value: 1.0,
                },
                probs,
                &BOUNDS
            );
            println!("market: {:?}", market);
            assert_slice_f64_relative(&[10.0, 5.0, 3.333, 2.5], &market.prices, 0.001);
        }
        {
            let probs = vec![0.1, 0.2, 0.3, 0.4];
            let market = Market::frame(
                &Overround {
                    method: OverroundMethod::Power,
                    value: 1.1,
                },
                probs,
                &BOUNDS
            );
            println!("market: {:?}", market);
            assert_slice_f64_relative(&[8.4319, 4.4381, 3.0489, 2.3359], &market.prices, 0.001);
        }
        {
            let probs = vec![0.1, 0.2, 0.3, 0.4, 0.0];
            let market = Market::frame(
                &Overround {
                    method: OverroundMethod::Power,
                    value: 1.1,
                },
                probs,
                &BOUNDS
            );
            println!("market: {:?}", market);
            assert_slice_f64_relative(
                &[8.4319, 4.4381, 3.0489, 2.3359, f64::INFINITY],
                &market.prices,
                0.001,
            );
        }
        {
            let probs = vec![0.2, 0.4, 0.6, 0.8];
            let market = Market::frame(
                &Overround {
                    method: OverroundMethod::Power,
                    value: 1.1,
                },
                probs,
                &BOUNDS
            );
            println!("market: {:?}", market);
            assert_slice_f64_relative(&[4.2159, 2.219, 1.5244, 1.168], &market.prices, 0.001);
        }
    }

    #[test]
    fn frame_odds_ratio() {
        {
            let probs = vec![0.1, 0.2, 0.3, 0.4];
            let market = Market::frame(
                &Overround {
                    method: OverroundMethod::OddsRatio,
                    value: 1.0,
                },
                probs,
                &BOUNDS
            );
            println!("market: {:?}", market);
            assert_slice_f64_relative(&[10.0, 5.0, 3.333, 2.5], &market.prices, 0.001);
        }
        {
            let probs = vec![0.1, 0.2, 0.3, 0.4];
            let market = Market::frame(
                &Overround {
                    method: OverroundMethod::OddsRatio,
                    value: 1.1,
                },
                probs,
                &BOUNDS
            );
            println!("market: {:?}", market);
            assert_slice_f64_relative(&[8.8335, 4.4816, 3.0309, 2.3056], &market.prices, 0.001);
        }
        {
            let probs = vec![0.1, 0.2, 0.3, 0.4, 0.0];
            let market = Market::frame(
                &Overround {
                    method: OverroundMethod::OddsRatio,
                    value: 1.1,
                },
                probs,
                &BOUNDS
            );
            println!("market: {:?}", market);
            assert_slice_f64_relative(
                &[8.8335, 4.4816, 3.0309, 2.3056, f64::INFINITY],
                &market.prices,
                0.001,
            );
        }
        {
            let probs = vec![0.2, 0.4, 0.6, 0.8];
            let market = Market::frame(
                &Overround {
                    method: OverroundMethod::OddsRatio,
                    value: 1.1,
                },
                probs,
                &BOUNDS
            );
            println!("market: {:?}", market);
            assert_slice_f64_relative(&[4.1132, 2.1675, 1.5189, 1.1946], &market.prices, 0.001);
        }
        {
            let probs = vec![0.1, 0.2, 0.3, 0.4, 0.0];
            let market = Market::frame(
                &Overround {
                    method: OverroundMethod::OddsRatio,
                    value: 3.6,
                },
                probs,
                &BOUNDS
            );
            println!("market: {:?}", market);
            assert_slice_f64_relative(
                &[1.2494, 1.1109, 1.0647, 1.0416, f64::INFINITY],
                &market.prices,
                0.001,
            );
        }
    }

    #[test]
    fn booksum() {
        let probs = vec![0.1, 0.2, 0.3, 0.4, 0.0];
        let market = Market::frame(&Overround {
            method: OverroundMethod::Multiplicative,
            value: 1.1,
        }, probs, &BOUNDS);
        assert_eq!(1.0, market.fair_booksum());
        assert_eq!(1.1, market.offered_booksum());
    }
}
