use crate::probs::SliceExt;
use crate::random;
use std::time::Instant;
use tinyrand::Rand;

/// Scale parameter for the exponential probability allocator.
const BETA: f64 = 0.25;

/// Simulation scenario.
#[derive(Debug)]
pub struct Scenario {
    /// Number of outcomes in a field.
    pub field: usize,

    /// Number of places to estimate.
    pub k: usize,
}

/// Simulation statistics.
#[derive(Default, Debug)]
pub struct Stats {
    pub samples: Vec<Errors>,
    pub mean: Errors,
}

impl Stats {
    pub fn quantiles(&self, field: impl Fn(&Errors) -> f64, quantiles: &[f64]) -> Vec<f64> {
        let sorted = self.sorted(&field);
        quantiles
            .iter()
            .map(|quantile| {
                let index = ((sorted.len() - 1) as f64 * quantile).ceil() as usize;
                field(&sorted[index])
            })
            .collect()
    }

    fn sorted(&self, field: impl Fn(&Errors) -> f64) -> Vec<Errors> {
        let mut copy = self.samples.clone();
        copy.sort_by(|lhs, rhs| field(lhs).total_cmp(&field(rhs)));
        copy
    }
}

#[derive(Default, Debug, Clone)]
pub struct Errors {
    pub rmse: f64,
    pub rmsre: f64,
}

/// Runs a complete simulation over a specified number of independent `trials` for the given `scenario`.
pub fn simulate(
    scenario: &Scenario,
    trials: usize,
    rand: &mut impl Rand,
    benchmark: &impl Fn(&[f64], usize) -> Vec<f64>,
    contender: &impl Fn(&[f64], usize) -> Vec<f64>,
) -> Stats {
    const RMSRE_WARN_THRESHOLD: f64 = 0.2;
    let mut samples = Vec::with_capacity(trials);
    let mut sum_sq_err = 0.0;
    let mut sum_sq_rel_err = 0.0;
    let start_time = Instant::now();
    for trial in 0..trials {
        let win_probs = generate_random_probs(scenario.field, rand);
        log::trace!("win_probs={win_probs:?}");

        let benchmark_place_probs = benchmark(&win_probs, scenario.k);
        log::trace!("benchmark_place_probs={benchmark_place_probs:?}");

        let contender_place_probs = contender(&win_probs, scenario.k);
        log::trace!("contender_place_probs={contender_place_probs:?}");

        if trial == 0 {
            let took = Instant::now() - start_time;
            log::debug!(
                "one iteration of {scenario:?} took {:.3}s, estimated time remaining: {:.0}s",
                took.as_secs_f64(),
                took.as_secs_f64() * (trials - 1) as f64
            );
        }

        let mut local_sum_sq_err = 0.0;
        let mut local_sum_sq_rel_err = 0.0;
        for (b, c) in benchmark_place_probs
            .iter()
            .zip(contender_place_probs.iter())
        {
            let err = b - c;
            local_sum_sq_err += err * err;
            let rel_err = err / b;
            local_sum_sq_rel_err += rel_err * rel_err;
        }
        sum_sq_err += local_sum_sq_err;
        sum_sq_rel_err += local_sum_sq_rel_err;

        let rmse = (local_sum_sq_err / win_probs.len() as f64).sqrt();
        let rmsre = (local_sum_sq_rel_err / win_probs.len() as f64).sqrt();
        samples.push(Errors { rmse, rmsre });
        if rmsre > RMSRE_WARN_THRESHOLD {
            log::warn!("RMSE={rmse}");
            log::warn!("RMSRE={rmsre}");
            log::warn!("win_probs={win_probs:?}");
            log::warn!("benchmark_place_probs={benchmark_place_probs:?}");
            log::warn!("contender_place_probs={contender_place_probs:?}");
        }
    }
    let summary = Errors {
        rmse: (sum_sq_err / trials as f64 / scenario.field as f64).sqrt(),
        rmsre: (sum_sq_rel_err / trials as f64 / scenario.field as f64).sqrt(),
    };
    Stats { samples, mean: summary }
}

fn generate_random_probs(field: usize, rand: &mut impl Rand) -> Vec<f64> {
    let mut probs = (0..field).map(|_| 0.0).collect::<Vec<_>>();
    probs.fill_random_probs_exp(rand, &random::gaussian_3_sigma, BETA, 1.0);
    probs
}
