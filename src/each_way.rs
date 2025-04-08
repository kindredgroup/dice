pub mod overbroke_sim;
pub mod probs_sim;

use crate::capture::Capture;
use crate::dilative::DilatedProbs;
use crate::harville::{classic, harville_est, poly_harville_summary, stacked_harville_summary, superstacked_harville_summary};
use crate::market::{Market, Overround, OverroundMethod, PriceBounds};
use crate::matrix::Matrix;
use crate::opt::{UnivariateDescentConfig, univariate_descent};
use crate::probs::SliceExt;
use std::ops::Div;

/// Converts win odds to place using naive (E/W) odds-ratio.
pub fn win_to_place_odds(win_odds: &[f64], d: usize) -> Vec<f64> {
    let d = d as f64;
    win_odds
        .iter()
        .map(|win_odds| (win_odds - 1.0) / d + 1.0)
        .collect()
}

/// Produces place probability estimates for `k` placings using the Booksum-Adjusted Odds-Ratio method.
pub fn win_to_baor_redist_place_probs(win_probs: &[f64], k: usize) -> Vec<f64> {
    let k = k as f64;
    let mut place_probs = win_probs
        .iter()
        .map(|win_prob| 1.0 / (((1.0 / win_prob) - 1.0) / k + 1.0))
        .collect::<Vec<_>>();
    place_probs.normalise(k);
    place_probs.redistribute();
    place_probs
}

/// Produces place probability estimates for `k` placings using the Dynamic Odds-Ratio method.
pub fn win_to_dynor_place_probs(win_probs: &[f64], k: usize) -> Vec<f64> {
    const BOUNDS: PriceBounds = 1.04..=10_001.0;

    let market = Market::frame(
        &Overround {
            method: OverroundMethod::OddsRatio,
            value: k as f64,
        },
        win_probs.iter().map(|p| *p).collect::<Vec<_>>(),
        &BOUNDS,
    );

    market.prices.invert().collect()
}

/// Produces place probability estimates for `k` placings using the Harville method.
pub fn win_to_harville_place_probs(win_probs: &[f64], k: usize) -> Vec<f64> {
    let win_probs = Matrix::from(
        DilatedProbs::default()
            .with_win_probs(Capture::Borrowed(win_probs))
            .with_podium_places(k),
    );
    let rank_probs = classic::summary(&win_probs);
    (0..rank_probs.cols())
        .map(|col| {
            (0..rank_probs.rows())
                .map(|row| rank_probs[(row, col)])
                .sum()
        })
        .collect()
}

/// Produces place probability estimates for `k` placings using an alternative Harville estimation method.
pub fn win_to_est_place_probs(win_probs: &[f64], k: usize) -> Vec<f64> {
    let all_rank_probs = (2..=k)
        .map(|rank| harville_est(&win_probs, rank, 1.0))
        .collect::<Vec<_>>();
    win_probs
        .iter()
        .enumerate()
        .map(|(index, win_prob)| {
            win_prob + all_rank_probs.iter().map(|probs| probs[index]).sum::<f64>()
        })
        .collect()
}

pub fn win_to_upscaled_place_probs(win_probs: &[f64], k: usize, fit_rank_idx: usize) -> Vec<f64> {
    let harville = classic::summary(
        &Matrix::from(
            DilatedProbs::default()
                .with_win_probs(Capture::Borrowed(win_probs))
                .with_podium_places(fit_rank_idx + 1),
        )
    );
    let harville = &harville[fit_rank_idx];
    let outcome = univariate_descent(
        &UnivariateDescentConfig {
            init_value: 1.1,
            init_step: 0.1,
            min_step: 0.001,
            max_steps: 1000,
            acceptable_residual: 0.0001,
        },
        |value| {
            let est = harville_est(&win_probs, fit_rank_idx, value);
            let sq_err = est
                .iter()
                .zip(harville.iter())
                .map(|(est, harv)| (est - harv).powi(2))
                .sum::<f64>();
            sq_err.div(est.len() as f64).sqrt()
        },
    );
    log::trace!("opt. outcome={outcome:?}");
    let all_rank_probs = (1..k)
        .map(|rank_idx| harville_est(&win_probs, rank_idx, outcome.optimal_value))
        .collect::<Vec<_>>();
    let mut place_probs = win_probs
        .iter()
        .enumerate()
        .map(|(index, win_prob)| {
            win_prob
                + all_rank_probs
                    .iter()
                    .map(|rank_probs| rank_probs[index])
                    .sum::<f64>()
        })
        .collect::<Vec<_>>();
    log::trace!("redistributing {place_probs:?}");
    place_probs.redistribute();
    place_probs
}

/// Produces place probability estimates for `k` placings using the poly-Harville method.
pub fn win_to_poly_harville_place_probs(win_probs: &[f64], k: usize, degree: usize) -> Vec<f64> {
    let win_probs = Matrix::from(
        DilatedProbs::default()
            .with_win_probs(Capture::Borrowed(win_probs))
            .with_podium_places(k),
    );
    let rank_probs = poly_harville_summary(&win_probs, k, degree);
    (0..rank_probs.cols())
        .map(|col| {
            (0..rank_probs.rows())
                .map(|row| rank_probs[(row, col)])
                .sum()
        })
        .collect()
}

pub fn win_to_stacked_harville_place_probs(win_probs: &[f64], k: usize, degree: usize) -> Vec<f64> {
    let win_probs = Matrix::from(
        DilatedProbs::default()
            .with_win_probs(Capture::Borrowed(win_probs))
            .with_podium_places(k),
    );
    let rank_probs = stacked_harville_summary(&win_probs, k, degree);
    let mut place_probs = (0..rank_probs.cols())
        .map(|col| {
            (0..rank_probs.rows())
                .map(|row| rank_probs[(row, col)])
                .sum()
        })
        .collect::<Vec<_>>();
    place_probs.redistribute();
    place_probs
}

pub fn win_to_superstacked_harville_place_probs(win_probs: &[f64], k: usize, degree: usize) -> Vec<f64> {
    let win_probs = Matrix::from(
        DilatedProbs::default()
            .with_win_probs(Capture::Borrowed(win_probs))
            .with_podium_places(k),
    );
    let rank_probs = superstacked_harville_summary(&win_probs, k, degree);
    let mut place_probs = (0..rank_probs.cols())
        .map(|col| {
            (0..rank_probs.rows())
                .map(|row| rank_probs[(row, col)])
                .sum()
        })
        .collect::<Vec<_>>();
    place_probs.redistribute();
    place_probs
}
