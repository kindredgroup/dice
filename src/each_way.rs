pub mod probs_sim;
pub mod overbroke_sim;

use crate::capture::Capture;
use crate::dilative::DilatedProbs;
use crate::harville::harville_summary_condensed;
use crate::market::{Market, Overround, OverroundMethod, PriceBounds};
use crate::matrix::Matrix;
use crate::probs::SliceExt;

/// Converts win odds to place using naive (E/W) odds-ratio.
pub fn win_to_place_odds(win_odds: &[f64], d: u8) -> Vec<f64> {
    let d = d as f64;
    win_odds
        .iter()
        .map(|win_odds| (win_odds - 1.0) / d + 1.0)
        .collect()
}

/// Produces place probability estimates for `k` placings using the Booksum-Adjusted Odds-Ratio method.
pub fn win_to_baor_place_probs(win_probs: &[f64], k: u8) -> Vec<f64> {
    let k = k as f64;
    let mut place_probs = win_probs
        .iter()
        .map(|win_prob| 1.0 / (((1.0 / win_prob) - 1.0) / k + 1.0))
        .collect::<Vec<_>>();
    place_probs.normalise(k);
    place_probs
}

/// Produces place probability estimates for `k` placings using the Dynamic Odds-Ratio method.
pub fn win_to_dynor_place_probs(win_probs: &[f64], k: u8) -> Vec<f64> {
    const BOUNDS: PriceBounds = 1.04..=10_001.0;
    
    let market = Market::frame(
        &Overround {
            method: OverroundMethod::OddsRatio,
            value: k as f64,
        },
        win_probs.iter().map(|p| *p).collect::<Vec<_>>(),
        &BOUNDS
    );
    
    market.prices.invert().collect()
}

/// Produces place probability estimates for `k` placings using the Harville method.
pub fn win_to_harville_place_probs(win_probs: &[f64], k: u8) -> Vec<f64> {
    let win_probs = Matrix::from(
        DilatedProbs::default()
            .with_win_probs(Capture::Borrowed(win_probs))
            .with_podium_places(k as usize),
    );
    harville_summary_condensed(&win_probs, k as usize)
}
