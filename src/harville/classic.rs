use crate::capture::CaptureMut;
use crate::comb::{count_permutations, is_unique_linear, pick_permutation, pick_state_hyper};
use crate::harville::harville;
use crate::matrix::Matrix;

pub struct Alloc<'a> {
    pub podium: CaptureMut<'a, Vec<usize>, [usize]>,
    pub bitmap:  CaptureMut<'a, Vec<bool>, [bool]>,
    pub summary:  CaptureMut<'a, Matrix<f64>>,
}

impl<'a> Alloc<'a> {
    pub fn new(runners: usize, ranks: usize) -> Self {
        Self {
            podium: vec![0; ranks].into(),
            bitmap: vec![false; runners].into(),
            summary: Matrix::allocate(ranks, runners).into(),
        }
    }
}

pub fn summary(probs: &Matrix<f64>) -> Matrix<f64> {
    let runners = probs.cols();
    let ranks = probs.rows();
    let mut alloc = Alloc::new(runners, ranks);
    summary_no_alloc(probs, &mut alloc);
    alloc.summary.into_owned()
}

pub fn summary_no_alloc(
    probs: &Matrix<f64>,
    alloc: &mut Alloc,
) {
    let Alloc { podium, bitmap, summary } = alloc;
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

    // the cost of traversing a permutation is n^2, while the cost of traversing a state is r, where
    // n is the number of runners and r is the number of ranks
    let permutations = count_permutations(runners, ranks);
    let states = runners.pow(ranks as u32);
    let perms_cost = permutations * runners * runners / 2;
    let states_cost = states * ranks;
    log::trace!("states: {states}, permutations: {permutations}, states cost: {states_cost}, perms cost: {perms_cost}, using {}", if states_cost < perms_cost { "states" } else { "perms" });

    if states_cost < perms_cost {
        // traversing states is cheaper
        for state in 0..states {
            pick_state_hyper(runners, ranks, state, podium);
            if !is_unique_linear(podium, bitmap) {
                continue;
            }
            let prob = harville(probs, podium);
            for (rank, &runner) in podium.iter().enumerate() {
                summary[(rank, runner)] += prob;
            }
        }
    } else {
        // traversing permutations is cheaper
        for permutation in 0..permutations {
            pick_permutation(runners, permutation, bitmap, podium);
            let prob = harville(probs, podium);
            for (rank, &runner) in podium.iter().enumerate() {
                summary[(rank, runner)] += prob;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_float_eq::assert_float_relative_eq;
    use crate::capture::Capture;
    use crate::dilative::DilatedProbs;
    use crate::harville::classic::summary;
    use crate::matrix::Matrix;
    use crate::testing::assert_slice_f64_relative;
    use crate::probs::SliceExt;

    #[test]
    fn harville_summary_3x3_without_scratchings() {
        const WIN_PROBS: [f64; 3] = [0.6, 0.3, 0.1];
        const RANKS: usize = 3;
        let probs = Matrix::from(
            DilatedProbs::default()
                .with_win_probs(Capture::Borrowed(&WIN_PROBS))
                .with_podium_places(RANKS),
        );
        let summary = summary(&probs);
        println!("summary:\n{}", summary.verbose());
        assert_slice_f64_relative(
            &[
                0.6,
                0.3,
                0.1,
                0.32380952380952444,
                0.48333333333333445,
                0.19285714285714314,
                0.07619047619047627,
                0.216666666666667,
                0.7071428571428587,
            ],
            summary.flatten(),
            1e-9,
        );

        for row in summary.into_iter() {
            assert_float_relative_eq!(1.0, row.sum());
        }
        for col in 0..summary.cols() {
            let col_cells = summary.col(col);
            assert_float_relative_eq!(1.0, col_cells.sum::<f64>());
        }
    }

    #[test]
    fn harville_summary_3x4_with_scratching() {
        const WIN_PROBS: [f64; 4] = [0.6, 0.3, 0.1, 0.0];
        const RANKS: usize = 3;
        let probs = Matrix::from(
            DilatedProbs::default()
                .with_win_probs(Capture::Borrowed(&WIN_PROBS))
                .with_podium_places(RANKS),
        );
        let summary = summary(&probs);
        println!("summary:\n{}", summary.verbose());
        assert_slice_f64_relative(
            &[
                0.6,
                0.3,
                0.1,
                0.0,
                0.32380952380952444,
                0.48333333333333445,
                0.19285714285714314,
                0.0,
                0.07619047619047627,
                0.216666666666667,
                0.7071428571428587,
                0.0,
            ],
            summary.flatten(),
            1e-9,
        );

        for row in summary.into_iter() {
            assert_float_relative_eq!(1.0, row.sum());
        }
        for col in 0..summary.cols() {
            let col_cells = summary.col(col);
            if col == 3 {
                assert_float_relative_eq!(0.0, col_cells.sum::<f64>());
            } else {
                assert_float_relative_eq!(1.0, col_cells.sum::<f64>());
            }
        }
    }

    #[test]
    fn harville_summary_4x4_without_scratchings() {
        const WIN_PROBS: [f64; 4] = [0.4, 0.3, 0.2, 0.1];
        const RANKS: usize = 4;
        let probs = Matrix::from(
            DilatedProbs::default()
                .with_win_probs(Capture::Borrowed(&WIN_PROBS))
                .with_podium_places(RANKS),
        );
        let summary = summary(&probs);
        assert_eq!(RANKS, summary.rows());
        assert_eq!(WIN_PROBS.len(), summary.cols());
        assert_slice_f64_relative(&WIN_PROBS, &summary[0], 1e-9);
        println!("summary:\n{}", summary.verbose());

        for row in summary.into_iter() {
            assert_float_relative_eq!(1.0, row.sum());
        }
        for col in 0..summary.cols() {
            let col_cells = summary.col(col);
            assert_float_relative_eq!(1.0, col_cells.sum::<f64>());
        }
    }

    #[test]
    fn harville_summary_1x4_without_scratchings() {
        const WIN_PROBS: [f64; 4] = [0.4, 0.3, 0.2, 0.1];
        const RANKS: usize = 1;
        let probs = Matrix::from(
            DilatedProbs::default()
                .with_win_probs(Capture::Borrowed(&WIN_PROBS))
                .with_podium_places(RANKS),
        );
        let summary = summary(&probs);
        assert_eq!(RANKS, summary.rows());
        assert_eq!(WIN_PROBS.len(), summary.cols());
        assert_slice_f64_relative(&WIN_PROBS, &summary[0], 1e-9);
        println!("summary:\n{}", summary.verbose());

        for row in summary.into_iter() {
            assert_float_relative_eq!(1.0, row.sum());
        }
        for col in 0..summary.cols() {
            let col_cells = summary.col(col);
            assert!(col_cells.sum::<f64>() <= 1.0);
        }
    }

    #[test]
    fn harville_summary_2x4_without_scratchings() {
        const WIN_PROBS: [f64; 4] = [0.4, 0.3, 0.2, 0.1];
        const RANKS: usize = 2;
        let probs = Matrix::from(
            DilatedProbs::default()
                .with_win_probs(Capture::Borrowed(&WIN_PROBS))
                .with_podium_places(RANKS),
        );
        let summary = summary(&probs);
        assert_eq!(RANKS, summary.rows());
        assert_eq!(WIN_PROBS.len(), summary.cols());
        assert_slice_f64_relative(&WIN_PROBS, &summary[0], 1e-9);
        println!("summary:\n{}", summary.verbose());

        for row in summary.into_iter() {
            assert_float_relative_eq!(1.0, row.sum());
        }
        for col in 0..summary.cols() {
            let col_cells = summary.col(col);
            assert!(col_cells.sum::<f64>() <= 1.0);
        }
    }

    #[test]
    fn harville_summary_4x4_without_scratchings_dilated() {
        const WIN_PROBS: [f64; 4] = [0.4, 0.3, 0.2, 0.1];
        const DILATIVES: [f64; 4] = [0.0, 0.1, 0.2, 0.3];
        const RANKS: usize = 4;
        let probs = Matrix::from(
            DilatedProbs::default()
                .with_win_probs(Capture::Borrowed(&WIN_PROBS))
                .with_dilatives(Capture::Borrowed(&DILATIVES)),
        );
        let summary = summary(&probs);
        assert_eq!(RANKS, summary.rows());
        assert_eq!(WIN_PROBS.len(), summary.cols());
        assert_slice_f64_relative(&WIN_PROBS, &summary[0], 1e-9);
        println!("summary:\n{}", summary.verbose());

        for row in summary.into_iter() {
            assert_float_relative_eq!(1.0, row.sum());
        }
        for col in 0..summary.cols() {
            let col_cells = summary.col(col);
            assert_float_relative_eq!(1.0, col_cells.sum::<f64>());
        }
    }
}