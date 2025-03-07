use crate::each_way::{win_to_est_place_probs, win_to_place_odds};
use crate::probs::SliceExt;
use tinyrand::Rand;

/// Summary of the overall simulation.
#[derive(Default, Debug)]
pub struct Stats {
    pub total_overbroke: usize,
    pub total_under_target_booksum: usize,
    pub total_at_least_one_value_outcome: usize,
    pub total_value_outcomes: usize,
}

/// Simulation scenario.
#[derive(Debug)]
pub struct Scenario {
    /// Number of outcomes in a field.
    pub field: usize,

    /// The overround for the win book.
    pub win_overround: f64,

    /// Number of places payable.
    pub k: u8,

    /// Odds split factor.
    pub d: u8,

    /// Target relative overround for the place book. The target booksum will be `k * target_place_overround`.
    pub target_place_overround: f64,
}

/// Runs a complete simulation over a specified number of `cycles` for the given `scenario`.
pub fn simulate(scenario: &Scenario, cycles: usize, rand: &mut impl Rand) -> Stats {
    let mut stats = Stats::default();
    for _ in 0..cycles {
        let result = simulate_one(scenario, rand);
        log::trace!("result={result:?}");
        if result.overbroke {
            stats.total_overbroke += 1;
        }
        if result.under_target_booksum {
            stats.total_under_target_booksum += 1;
        }
        if result.value_outcomes > 0 {
            stats.total_at_least_one_value_outcome += 1;
            stats.total_value_outcomes += result.value_outcomes;
        }
    }
    stats
}

#[derive(Debug)]
struct SimulationResult {
    overbroke: bool,
    under_target_booksum: bool,
    value_outcomes: usize,
}

fn simulate_one(scenario: &Scenario, rand: &mut impl Rand) -> SimulationResult {
    let win_probs = generate_random_probs(scenario.field, rand);
    log::trace!("win_probs={win_probs:?}, sum={}", win_probs.sum());

    let win_odds = probs_to_odds(&win_probs, scenario.win_overround);
    log::trace!("win_odds={win_odds:?}, booksum={}", win_odds.booksum());

    let place_odds = win_to_place_odds(&win_odds, scenario.d);
    log::trace!(
        "place_odds={place_odds:?}, booksum={}",
        place_odds.booksum()
    );

    let place_probs = win_to_est_place_probs(&win_probs, scenario.k);
    log::trace!("place_probs={place_probs:?}, sum={}", place_probs.sum());

    let place_prices = probs_to_prices(&place_probs);
    log::trace!(
        "place_prices={place_prices:?}, booksum={}",
        place_prices.booksum()
    );

    let booksum = place_odds.booksum();
    let overbroke = booksum < scenario.k as f64;
    let min_booksum = scenario.k as f64 * scenario.target_place_overround;
    let under_target_booksum = booksum < min_booksum;
    let value_outcomes = place_prices
        .iter()
        .zip(place_odds)
        .filter(|(price, odds)| price < &odds)
        .count();
    SimulationResult {
        overbroke,
        under_target_booksum,
        value_outcomes,
    }
}

fn generate_random_probs(field: usize, rand: &mut impl Rand) -> Vec<f64> {
    let mut probs = (0..field).map(|_| 0.0).collect::<Vec<_>>();
    probs.fill_random_probs(rand, 1.0);
    probs
}

/// Inverts true probs to obtain fair prices.
fn probs_to_prices(probs: &[f64]) -> Vec<f64> {
    probs.invert().collect()
}

/// Obtains offered odds from true probs.
fn probs_to_odds(probs: &[f64], overround: f64) -> Vec<f64> {
    let mut odds = probs_to_prices(&probs);
    odds.scale(1.0 / overround);
    odds
}
