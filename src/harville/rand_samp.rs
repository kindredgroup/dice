use crate::capture::CaptureMut;
use crate::comb::{count_permutations, pick_permutation};
use crate::harville::harville;
use crate::matrix::Matrix;
use crate::probs::SliceExt;
use std::cmp::max;
use tinyrand::{Rand, StdRand};

pub struct Alloc<'a, R: Rand> {
    pub podium: CaptureMut<'a, Vec<usize>, [usize]>,
    pub bitmap:  CaptureMut<'a, Vec<bool>, [bool]>,
    pub rand: CaptureMut<'a, R>,
    pub summary:  CaptureMut<'a, Matrix<f64>>,
}

impl<'a> Alloc<'a, StdRand> {
    pub fn new(runners: usize, ranks: usize) -> Self {
        Self {
            podium: vec![0; ranks].into(),
            bitmap: vec![false; runners].into(),
            rand: StdRand::default().into(),
            summary: Matrix::allocate(ranks, runners).into(),
        }
    }
}

pub fn summary(probs: &Matrix<f64>, degree: usize) -> Matrix<f64> {
    let runners = probs.cols();
    let ranks = probs.rows();
    let mut alloc = Alloc::new(runners, ranks);
    summary_no_alloc(probs, degree, &mut alloc);
    alloc.summary.into_owned()
}

pub fn summary_no_alloc<R: Rand>(
    probs: &Matrix<f64>,
    degree: usize,
    alloc: &mut Alloc<R>,
) {
    let Alloc { podium, bitmap, rand, summary } = alloc;
    debug_assert_eq!(
        summary.rows(),
        probs.rows(),
        "number of rows in the probabilities matrix must equal to the number of rows in the summary matrix"
    );
    debug_assert_eq!(
        summary.cols(),
        probs.cols(),
        "number of columns in the probabilities matrix must equal to the number of columns in the summary matrix"
    );
    debug_assert_eq!(
        probs.rows(),
        podium.len(),
        "number of rows in the probabilities matrix must equal to the podium length"
    );
    debug_assert_eq!(
        probs.cols(),
        bitmap.len(),
        "number of columns in the probabilities matrix must equal to the bitmap length"
    );
    let ranks = probs.rows();
    let runners = probs.cols();
    let total_permutations = count_permutations(runners, ranks);
    let capped_permutations = runners.pow(degree as u32);
    let step = max(1, total_permutations / capped_permutations);

    let mut permutation = 0;
    let mut evaluated = 0;
    while permutation < total_permutations {
        pick_permutation(runners, permutation, bitmap, podium);
        let jump = if step > 1 {
            rand.next_usize() % (step * 2) + 1
        } else {
            1
        };
        evaluated += 1;
        permutation += jump;
        let prob = harville(probs, podium);
        for (rank, &runner) in podium.iter().enumerate() {
            summary[(rank, runner)] += prob;
        }
    }
    log::trace!(
        "runners: {runners}, ranks: {ranks}, degree: {degree}, total perms: {total_permutations}, capped perms: {capped_permutations}, step: {step}, evaluated: {evaluated} ({:.6}%)",
        evaluated as f64 / total_permutations as f64 * 100.0
    );

    if step > 1 {
        for row_idx in 0..summary.rows() {
            summary.row_slice_mut(row_idx).normalise(1.0);
        }
    }
}