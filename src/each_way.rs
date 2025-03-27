pub mod probs_sim;
pub mod overbroke_sim;

use std::ops::Div;
use crate::capture::Capture;
use crate::dilative::DilatedProbs;
use crate::harville::{harville_est, old_harville_summary, harville_summary_condensed};
use crate::market::{Market, Overround, OverroundMethod, PriceBounds};
use crate::matrix::Matrix;
use crate::opt::{univariate_descent, UnivariateDescentConfig};
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

/// Produces place probability estimates for `k` placings using an alternative Harville estimation method.
pub fn win_to_est_place_probs(win_probs: &[f64], k: u8) -> Vec<f64> {
    let all_rank_probs = (2..=k).map(|rank| harville_est(&win_probs, rank as usize, 1.0)).collect::<Vec<_>>();
    win_probs.iter().enumerate().map(|(index, win_prob)| {
        win_prob + all_rank_probs.iter().map(|probs| probs[index]).sum::<f64>()
    }).collect()
}

pub fn win_to_opt_place_probs(win_probs: &[f64], k: u8, fit_rank_idx: u8) -> Vec<f64> {
    let harville = old_harville_summary(&Matrix::from(
        DilatedProbs::default()
            .with_win_probs(Capture::Borrowed(win_probs))
            .with_podium_places(fit_rank_idx as usize + 1),
    ), fit_rank_idx as usize + 1);
    let harville = &harville[fit_rank_idx as usize];
    let outcome = univariate_descent(&UnivariateDescentConfig {
        init_value: 1.1,
        init_step: 0.1,
        min_step: 0.001,
        max_steps: 1000,
        acceptable_residual: 0.0001
    }, |value| {
        let est = harville_est(&win_probs, fit_rank_idx as usize, value);
        let sq_err = est.iter().zip(harville.iter()).map(|(est, harv)| (est - harv).powi(2)).sum::<f64>();
        sq_err.div(est.len() as f64).sqrt()
    });
    log::trace!("opt. outcome={outcome:?}");
    let all_rank_probs = (1..k).map(|rank_idx| harville_est(&win_probs, rank_idx as usize, outcome.optimal_value)).collect::<Vec<_>>();
    let mut place_probs = win_probs.iter().enumerate().map(|(index, win_prob)| {
        win_prob + all_rank_probs.iter().map(|rank_probs| rank_probs[index]).sum::<f64>()
    }).collect::<Vec<_>>();
    place_probs.redistribute();
    place_probs
}