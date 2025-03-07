use tinyrand::{Rand, StdRand};
use dice::probs::SliceExt;

const FIELD: usize = 20;
const OVERROUND: f64 = 1.15;
const K: u8 = 3;
const D: u8 = 3;

fn main() {
    env_logger::init();
    
    let mut rand = StdRand::default();
    simulate(&mut rand);
}

struct SimulationOutcome {
    overbroke: bool,
    underTargetBooksum: bool,
    valueOutcomes: usize
}

fn simulate(rand: &mut impl Rand) {
    let win_probs = generate_random_probs::<FIELD>(rand);
    log::trace!("win_probs={win_probs:?}, sum={}", win_probs.sum());
    
    let win_odds = probs_to_odds(&win_probs, OVERROUND);
    log::trace!("win_odds={win_odds:?}, booksum={}", booksum(&win_odds));
    
    let place_odds = win_to_place_odds(&win_odds, D);
    log::trace!("place_odds={place_odds:?}, booksum={}", booksum(&place_odds));
    
    let place_probs = win_to_est_place_probs(&win_probs, K);
    log::trace!("place_probs={place_probs:?}, sum={}", place_probs.sum());
    
    let place_prices = probs_to_prices(&place_probs);
    log::trace!("place_prices={place_prices:?}, booksum={}", booksum(&place_prices));
}

fn generate_random_probs<const N: usize>(rand: &mut impl Rand) -> [f64; N] {
    let mut probs = [0.0; N];
    for prob in &mut probs {
        *prob = random_f64(rand);
    }
    probs.normalise(1.0);
    probs
}

// Inverts true probs to obtain fair prices.
fn probs_to_prices<const N: usize>(probs: &[f64; N]) -> [f64; N] {
    let mut prices = [0.0; N];
    for (i, prob) in probs.iter().enumerate() {
        prices[i] = 1.0 / prob;
    }
    prices
}

// Obtains offered odds from true probs.
fn probs_to_odds<const N: usize>(probs: &[f64; N], overround: f64) -> [f64; N] {
    let mut odds = probs_to_prices(&probs);
    odds.scale(1.0 / overround);
    odds
}

// Converts win odds to place using naive (E/W) odds-ratio.
fn win_to_place_odds<const N: usize>(win_odds: &[f64; N], d: u8) -> [f64; N] {
    let mut place_odds = [0.0; N];
    let d = d as f64;
    for (i, win_odds) in win_odds.iter().enumerate() {
        place_odds[i] = (win_odds - 1.0) / d + 1.0;
    }
    place_odds
}

fn win_to_est_place_probs<const N: usize>(win_probs: &[f64; N], k: u8) -> [f64; N] {
    let mut place_probs =  [0.0; N];
    let k = k as f64;
    for (i, win_prob) in win_probs.iter().enumerate() {
        place_probs[i] = 1.0 / (((1.0 / win_prob) - 1.0) / k + 1.0);
    }
    place_probs.normalise(k);
    place_probs
}

fn booksum(odds: &[f64]) -> f64 {
    odds.invert().sum()
}

#[inline(always)]
fn random_f64(rand: &mut impl Rand) -> f64 {
    rand.next_u64() as f64 / u64::MAX as f64
}