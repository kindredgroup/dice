use crate::probs::SliceExt;
use tinyrand::Rand;

/// Scale parameter for the exponential probability allocator.
const BETA: f64 = 0.25;

/// Simulation scenario.
#[derive(Debug)]
pub struct Scenario {
    /// Number of outcomes in a field.
    pub field: usize,

    /// Number of places payable.
    pub k: u8,
}

/// Summary of the overall simulation.
#[derive(Default, Debug)]
pub struct Stats {
    pub rmse: f64,
    pub rmsre: f64,
}

/// Runs a complete simulation over a specified number of independent `trials` for the given `scenario`.
pub fn simulate(
    scenario: &Scenario,
    trials: usize,
    rand: &mut impl Rand,
    benchmark: &impl Fn(&[f64], u8) -> Vec<f64>,
    contender: &impl Fn(&[f64], u8) -> Vec<f64>,
) -> Stats {
    let mut sum_sq_err = 0.0;
    let mut sum_sq_rel_err = 0.0;
    for _ in 0..trials {
        let win_probs = generate_random_probs(scenario.field, rand);
        log::trace!("win_probs={win_probs:?}");

        let benchmark_place_probs = benchmark(&win_probs, scenario.k);
        log::trace!("benchmark_place_probs={benchmark_place_probs:?}");

        let contender_place_probs = contender(&win_probs, scenario.k);
        log::trace!("contender_place_probs={contender_place_probs:?}");

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

        if (local_sum_sq_rel_err / win_probs.len() as f64).sqrt() > 0.1 {
            log::warn!("RMSE={}", (local_sum_sq_err / win_probs.len() as f64).sqrt());
            log::warn!("RMSRE={}", (local_sum_sq_rel_err / win_probs.len() as f64).sqrt());
            log::warn!("win_probs={win_probs:?}");
            log::warn!("benchmark_place_probs={benchmark_place_probs:?}");
            log::warn!("contender_place_probs={contender_place_probs:?}");
        }
    }
    Stats {
        rmse: (sum_sq_err / trials as f64 / scenario.field as f64).sqrt(),
        rmsre: (sum_sq_rel_err / trials as f64 / scenario.field as f64).sqrt(),
    }
}

fn generate_random_probs(field: usize, rand: &mut impl Rand) -> Vec<f64> {
    let mut probs = (0..field).map(|_| 0.0).collect::<Vec<_>>();
    probs.fill_random_probs_exp(rand, BETA, 1.0);
    probs
}
