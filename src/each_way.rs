pub mod sim;

use crate::probs::SliceExt;

/// Converts win odds to place using naive (E/W) odds-ratio.
fn win_to_place_odds(win_odds: &[f64], d: u8) -> Vec<f64> {
    let d = d as f64;
    win_odds
        .iter()
        .map(|win_odds| (win_odds - 1.0) / d + 1.0)
        .collect()
}

/// Produces place probability estimates for `k` placings using the Booksum-Adjusted Odds-Ratio method.
fn win_to_est_place_probs(win_probs: &[f64], k: u8) -> Vec<f64> {
    let k = k as f64;
    let mut place_probs = win_probs
        .iter()
        .map(|win_prob| 1.0 / (((1.0 / win_prob) - 1.0) / k + 1.0))
        .collect::<Vec<_>>();
    place_probs.normalise(k);
    place_probs
}
