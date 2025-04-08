use crate::capture::CaptureMut;
use crate::comb::generator::Generator;
use crate::comb::permuter::Permuter;
use crate::comb::{count_permutations, permuter, pick_permutation, sticky};
use crate::matrix::Matrix;
use crate::probs::SliceExt;
use std::cmp::max;
use tinyrand::{Rand, StdRand};

pub mod classic;

#[inline]
pub fn harville(probs: &Matrix<f64>, podium: &[usize]) -> f64 {
    let mut combined_prob = 1.;
    // println!("probs: {probs:?}, podium: {podium:?}");
    // for (rank, rank_probs) in probs.into_iter().enumerate() {
    for rank in 0..podium.len() {
        let rank_probs = probs.row_slice(rank);
        let runner = podium[rank];
        let mut remaining_prob = 1.;
        for prev_rank in 0..rank {
            remaining_prob -= rank_probs[podium[prev_rank]];
        }
        let prob = rank_probs[runner];
        combined_prob *= prob / remaining_prob;
        // println!("  rank: {rank}, prob: {prob}, combined_prob: {combined_prob}, remaining_prob: {remaining_prob}");
    }
    combined_prob
}



pub fn poly_harville_summary(probs: &Matrix<f64>, ranks: usize, degree: usize) -> Matrix<f64> {
    let runners = probs.cols();
    let mut summary = Matrix::allocate(ranks, runners);
    let mut podium = vec![0; ranks];
    let mut bitmap = vec![false; runners];
    let mut rand = StdRand::default();
    poly_harville_summary_no_alloc(
        probs,
        ranks,
        degree,
        &mut podium,
        &mut bitmap,
        &mut rand,
        &mut summary,
    );
    summary
}

pub fn poly_harville_summary_no_alloc(
    probs: &Matrix<f64>,
    ranks: usize,
    degree: usize,
    podium: &mut [usize],
    bitmap: &mut [bool],
    rand: &mut impl Rand,
    summary: &mut Matrix<f64>,
) {
    debug_assert_eq!(
        probs.rows(),
        ranks,
        "number of rows in the probabilities matrix must equal to the number of ranks"
    );
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
    let runners = probs.cols();
    let total_permutations = count_permutations(runners, ranks);
    let capped_permutations = runners.pow(degree as u32);
    let step = max(1, total_permutations / capped_permutations);

    let mut permutation = 0;
    let mut evaluated = 0;
    while permutation < total_permutations {
        pick_permutation(runners, permutation, bitmap, podium);
        let jump = if step > 1 {
            // rand.next_lim_usize(step * 2) + 1
            rand.next_usize() % (step * 2) + 1
        } else {
            1
        };
        // println!("jump={jump}");
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
        // // assign the win probs from the source
        // for (index, prob) in summary.row_slice_mut(0).iter_mut().enumerate() {
        //     *prob = probs[(0, index)]
        // }
        for row_idx in 0..summary.rows() {
            summary.row_slice_mut(row_idx).normalise(1.0);
        }
    }
}

pub fn stacked_harville_summary(probs: &Matrix<f64>, ranks: usize, degree: usize) -> Matrix<f64> {
    let runners = probs.cols();
    let mut summary = Matrix::allocate(ranks, runners);
    let mut podium = vec![0; ranks];
    let mut bitmap = vec![false; runners - 1];
    stacked_harville_summary_no_alloc(
        probs,
        ranks,
        degree,
        &mut podium,
        &mut bitmap,
        &mut summary,
    );
    summary
}

pub fn stacked_harville_summary_no_alloc(
    probs: &Matrix<f64>,
    ranks: usize,
    degree: usize,
    podium: &mut [usize],
    bitmap: &mut [bool],
    summary: &mut Matrix<f64>,
) {
    debug_assert_eq!(
        probs.rows(),
        ranks,
        "number of rows in the probabilities matrix must equal to the number of ranks"
    );
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
        probs.cols() - 1,
        bitmap.len(),
        "bitmap length must be one less than the number of columns in the probabilities matrix"
    );
    let runners = probs.cols();

    //TODO remove allocations
    let mut sorted_runners = (0..runners).collect::<Vec<_>>();

    //TODO can live with a smaller bitmap (n - 1 in size)

    //TODO can also live with a smaller podium

    // sort runners in decreasing order of win probability
    sorted_runners.sort_unstable_by(|a, b| {
        let a_prob = probs[(0, *a)];
        let b_prob = probs[(0, *b)];
        b_prob.total_cmp(&a_prob)
    });

    let quota = count_permutations(runners - 1, degree - 1);

    // let ranks = 2;//TODO
    for rank in 1..ranks {
        // let full_podium = vec![0; rank + 0]; // allocation
        let mut sans_self_podium = vec![0; rank]; //TODO
        let podium = &mut podium[..rank + 1];

        for runner in 0..runners {
            let sans_self = sorted_runners.iter().filter(|&&index| index != runner).collect::<Vec<_>>();
            let total_permutations = count_permutations(runners - 1, rank);
            log::trace!("runner: {runner}: rank: {rank}, total_perms: {total_permutations}, quota: {quota}, sans_self: {sans_self:?}");

            let mut permuter = Permuter::new_no_alloc(rank, permuter::Alloc {
                bitmap: CaptureMut::Borrowed(bitmap),
                ordinals: CaptureMut::Borrowed(&mut sans_self_podium),
            });
            let mut permutation = 0;
            loop {
                for (index, ordinal) in &mut permuter.ordinals().iter().enumerate() {
                    podium[index] = *sans_self[*ordinal];
                }
                podium[rank] = runner;
                let prob = harville(probs, podium);
                log::trace!("  podium: {podium:?}, prob: {prob:.6}");
                summary[(rank, runner)] += prob;

                if permutation == quota {
                    break;
                }

                if !permuter.advance() {
                    break;
                }
                
                permutation +=1;
            }
        }
    }

    // assign the win probs from the source
    for (index, prob) in summary.row_slice_mut(0).iter_mut().enumerate() {
        *prob = probs[(0, index)]
    }

    // normalise remaining rows (2nd rank onwards)
    for row_idx in 1..summary.rows() {
        summary.row_slice_mut(row_idx).normalise(1.0);
    }
}

pub fn superstacked_harville_summary(probs: &Matrix<f64>, ranks: usize, degree: usize) -> Matrix<f64> {
    let runners = probs.cols();
    let mut summary = Matrix::allocate(ranks, runners);
    let mut podium = vec![0; ranks];
    let mut bitmap = vec![false; runners - 1];
    superstacked_harville_summary_no_alloc(
        probs,
        ranks,
        degree,
        &mut podium,
        &mut bitmap,
        &mut summary,
    );
    summary
}

pub fn superstacked_harville_summary_no_alloc(
    probs: &Matrix<f64>,
    ranks: usize,
    degree: usize,
    podium: &mut [usize],
    bitmap: &mut [bool],
    summary: &mut Matrix<f64>,
) {
    debug_assert_eq!(
        probs.rows(),
        ranks,
        "number of rows in the probabilities matrix must equal to the number of ranks"
    );
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
        probs.cols() - 1,
        bitmap.len(),
        "bitmap length must be one less than the number of columns in the probabilities matrix"
    );
    let runners = probs.cols();

    //TODO remove allocations
    let mut sorted_runners = (0..runners).collect::<Vec<_>>();

    //TODO can live with a smaller bitmap (n - 1 in size)

    //TODO can also live with a smaller podium

    // sort runners in decreasing order of win probability
    sorted_runners.sort_unstable_by(|a, b| {
        let a_prob = probs[(0, *a)];
        let b_prob = probs[(0, *b)];
        b_prob.total_cmp(&a_prob)
    });

    let quota = count_permutations(runners - 1, degree - 1);

    // let ranks = 2;//TODO
    for rank in 1..ranks {
        // let full_podium = vec![0; rank + 0]; // allocation
        // let mut sans_self_podium = vec![0; rank]; //TODO
        let podium = &mut podium[..rank + 1];

        for runner in 0..runners {
            let sans_self = sorted_runners.iter().filter(|&&index| index != runner).collect::<Vec<_>>();
            let total_permutations = count_permutations(runners - 1, rank);
            log::trace!("runner: {runner}: rank: {rank}, total_perms: {total_permutations}, quota: {quota}, sans_self: {sans_self:?}");

            let mut permutation = 0;
            sticky::permute(runners - 1, rank, |ordinals| {
                for (index, ordinal) in &mut ordinals.iter().enumerate() {
                    podium[index] = *sans_self[*ordinal];
                }
                podium[rank] = runner;
                let prob = harville(probs, podium);
                log::trace!("  podium: {podium:?}, prob: {prob:.6}");
                summary[(rank, runner)] += prob;
                
                permutation += 1;
                permutation < quota
            });
        }
    }

    // assign the win probs from the source
    for (index, prob) in summary.row_slice_mut(0).iter_mut().enumerate() {
        *prob = probs[(0, index)]
    }

    // normalise remaining rows (2nd rank onwards)
    for row_idx in 1..summary.rows() {
        summary.row_slice_mut(row_idx).normalise(1.0);
    }
}

pub fn harville_est(probs: &[f64], rank_idx: usize, lambda: f64) -> Vec<f64> {
    let len_sub_1 = probs.len() as f64 - 1.0;
    let mut rank_probs = probs
        .iter()
        .map(|win_prob| {
            let r = ((1.0 - win_prob) / len_sub_1).powf(lambda);
            let numer = r.powi(rank_idx as i32) * win_prob;
            let denom = (2..=rank_idx + 1)
                .map(|j| 1.0 - r.powi((j - 1) as i32))
                .product::<f64>();
            //println!("r={r}, numer={numer}, denom={denom}");
            numer / denom
        })
        .collect::<Vec<_>>();
    rank_probs.normalise(1.0);
    rank_probs
}

#[cfg(test)]
mod tests {
    use crate::itemiser::Itemiser;
    use crate::testing::assert_slice_f64_relative;
    use assert_float_eq::assert_float_relative_eq;

    use crate::capture::Capture;
    use crate::comb::enumerator::Enumerator;
    use crate::comb::is_unique_quadratic;
    use crate::dilative::DilatedProbs;

    use super::*;

    #[derive(Debug, Clone)]
    struct PodiumProb {
        podium: Vec<usize>,
        prob: f64,
    }

    #[test]
    fn harville_3x3_without_scratchings() {
        const WIN_PROBS: [f64; 3] = [0.6, 0.3, 0.1];
        const RANKS: usize = 3;
        const RUNNERS: usize = WIN_PROBS.len();
        let probs = Matrix::from(
            DilatedProbs::default()
                .with_win_probs(Capture::Borrowed(&WIN_PROBS))
                .with_podium_places(3),
        );
        let enumerator = Enumerator::new(&[RUNNERS; RANKS]);
        let probs = enumerator
            .into_iter()
            .filter(|podium| is_unique_quadratic(&podium))
            .map(|podium| {
                let prob = harville(&probs, &podium);
                PodiumProb { podium, prob }
            })
            .collect::<Vec<_>>();
        assert_eq!(6, probs.len());

        let sum = probs
            .iter()
            .map(|podium_prob| podium_prob.prob)
            .sum::<f64>();
        assert_float_relative_eq!(1.0, sum);
    }

    #[test]
    fn harville_3x4_with_scratching() {
        const WIN_PROBS: [f64; 4] = [0.6, 0.3, 0.1, 0.0];
        const RANKS: usize = 3;
        const RUNNERS: usize = WIN_PROBS.len();
        let probs = Matrix::from(
            DilatedProbs::default()
                .with_win_probs(Capture::Borrowed(&WIN_PROBS))
                .with_podium_places(RANKS),
        );
        let enumerator = Enumerator::new(&[RUNNERS; RANKS]);
        let probs = enumerator
            .into_iter()
            .filter(|podium| is_unique_quadratic(&podium))
            .map(|podium| {
                let prob = harville(&probs, &podium);
                PodiumProb { podium, prob }
            })
            .collect::<Vec<_>>();
        assert_eq!(24, probs.len());

        let nonzero_scratched = probs
            .iter()
            .find(|&podium_prob| podium_prob.podium.contains(&3) && podium_prob.prob != 0.0);
        assert!(nonzero_scratched.is_none());

        let sum = probs
            .iter()
            .map(|podium_prob| podium_prob.prob)
            .sum::<f64>();
        assert_float_relative_eq!(1.0, sum);
    }

    #[test]
    fn harville_4x4_without_scratchings() {
        const WIN_PROBS: [f64; 4] = [0.4, 0.3, 0.2, 0.1];
        const RANKS: usize = 4;
        const RUNNERS: usize = WIN_PROBS.len();
        let probs = Matrix::from(
            DilatedProbs::default()
                .with_win_probs(Capture::Borrowed(&WIN_PROBS))
                .with_podium_places(RANKS),
        );
        let enumerator = Enumerator::new(&[RUNNERS; RANKS]);
        let probs = enumerator
            .into_iter()
            .filter(|podium| is_unique_quadratic(&podium))
            .map(|podium| {
                let prob = harville(&probs, &podium);
                PodiumProb { podium, prob }
            })
            .collect::<Vec<_>>();
        assert_eq!(24, probs.len());
        println!("probs: {probs:?}");

        let sum = probs
            .iter()
            .map(|podium_prob| podium_prob.prob)
            .sum::<f64>();
        assert_float_relative_eq!(1.0, sum);
    }

    #[test]
    fn harville_1x4_without_scratchings() {
        const WIN_PROBS: [f64; 4] = [0.6, 0.3, 0.1, 0.0];
        const RANKS: usize = 1;
        const RUNNERS: usize = WIN_PROBS.len();
        let probs = Matrix::from(
            DilatedProbs::default()
                .with_win_probs(Capture::Borrowed(&WIN_PROBS))
                .with_podium_places(RANKS),
        );
        let enumerator = Enumerator::new(&[RUNNERS; RANKS]);
        let probs = enumerator
            .into_iter()
            .filter(|podium| is_unique_quadratic(&podium))
            .map(|podium| {
                let prob = harville(&probs, &podium);
                PodiumProb { podium, prob }
            })
            .collect::<Vec<_>>();
        assert_eq!(4, probs.len());

        let sum = probs
            .iter()
            .map(|podium_prob| podium_prob.prob)
            .sum::<f64>();
        assert_float_relative_eq!(1.0, sum);
    }

    #[test]
    fn harville_2x4_without_scratchings() {
        const WIN_PROBS: [f64; 4] = [0.6, 0.3, 0.1, 0.0];
        const RANKS: usize = 2;
        const RUNNERS: usize = WIN_PROBS.len();
        let probs = Matrix::from(
            DilatedProbs::default()
                .with_win_probs(Capture::Borrowed(&WIN_PROBS))
                .with_podium_places(RANKS),
        );
        let enumerator = Enumerator::new(&[RUNNERS; RANKS]);
        let probs = enumerator
            .into_iter()
            .filter(|podium| is_unique_quadratic(&podium))
            .map(|podium| {
                let prob = harville(&probs, &podium);
                PodiumProb { podium, prob }
            })
            .collect::<Vec<_>>();
        assert_eq!(12, probs.len());

        let sum = probs
            .iter()
            .map(|podium_prob| podium_prob.prob)
            .sum::<f64>();
        assert_float_relative_eq!(1.0, sum);
    }

    #[test]
    fn harville_4x4_without_scratchings_dilated() {
        const WIN_PROBS: [f64; 4] = [0.4, 0.3, 0.2, 0.1];
        const DILATIVES: [f64; 4] = [0.0, 0.1, 0.2, 0.3];
        const RANKS: usize = 4;
        const RUNNERS: usize = WIN_PROBS.len();
        let probs = Matrix::from(
            DilatedProbs::default()
                .with_win_probs(Capture::Borrowed(&WIN_PROBS))
                .with_dilatives(Capture::Borrowed(&DILATIVES)),
        );
        let enumerator = Enumerator::new(&[RUNNERS; RANKS]);
        let probs = enumerator
            .into_iter()
            .filter(|podium| is_unique_quadratic(&podium))
            .map(|podium| {
                let prob = harville(&probs, &podium);
                PodiumProb { podium, prob }
            })
            .collect::<Vec<_>>();
        assert_eq!(24, probs.len());

        let sum = probs
            .iter()
            .map(|podium_prob| podium_prob.prob)
            .sum::<f64>();
        assert_float_relative_eq!(1.0, sum);
    }

    // Actual Harville for [0.4, 0.3, 0.2, 0.1]
    // [0.4000000000000016, 0.30000000000000104, 0.2000000000000007, 0.10000000000000028]
    // [0.315873015873017, 0.3083333333333345, 0.24126984126984216, 0.13452380952380988]
    // [0.2063492063492071, 0.2619047619047629, 0.3174603174603188, 0.21428571428571486]
    // [0.07777777777777792, 0.12976190476190508, 0.24126984126984197, 0.5511904761904786]

    #[test]
    fn harville_est_1x4() {
        const WIN_PROBS: [f64; 4] = [0.4, 0.3, 0.2, 0.1];
        let rank_probs = harville_est(&WIN_PROBS, 0, 1.0);
        assert_slice_f64_relative(&[0.4, 0.3, 0.2, 0.1], &rank_probs, 1e-9);
    }

    #[test]
    fn harville_est_2x4() {
        const WIN_PROBS: [f64; 4] = [0.4, 0.3, 0.2, 0.1];
        let rank_probs = harville_est(&WIN_PROBS, 1, 1.0);
        println!("rank_probs={rank_probs:?}");
        assert_slice_f64_relative(
            &[
                0.32585096596136154,
                0.2975160993560257,
                0.23698252069917203,
                0.13965041398344066,
            ],
            &rank_probs,
            1e-9,
        );
    }

    #[test]
    fn harville_est_3x4() {
        const WIN_PROBS: [f64; 4] = [0.4, 0.3, 0.2, 0.1];
        let rank_probs = harville_est(&WIN_PROBS, 2, 1.0);
        println!("rank_probs={rank_probs:?}");
        assert_slice_f64_relative(
            &[
                0.2658271045738685,
                0.28748930412403045,
                0.26640524094745244,
                0.18027835035464868,
            ],
            &rank_probs,
            1e-9,
        );
    }

    #[test]
    fn harville_est_4x4() {
        const WIN_PROBS: [f64; 4] = [0.4, 0.3, 0.2, 0.1];
        let rank_probs = harville_est(&WIN_PROBS, 3, 1.0);
        println!("rank_probs={rank_probs:?}");
        assert_slice_f64_relative(
            &[
                0.21477443749102987,
                0.2722801460997806,
                0.29019578417201364,
                0.2227496322371759,
            ],
            &rank_probs,
            1e-9,
        );
    }
}
