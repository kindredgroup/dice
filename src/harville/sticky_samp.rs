use crate::capture::CaptureMut;
use crate::comb::{count_permutations, sticky_permuter};
use crate::harville::harville;
use crate::matrix::Matrix;
use crate::probs::SliceExt;

pub struct Alloc<'a> {
    pub podium: CaptureMut<'a, Vec<usize>, [usize]>,
    pub sorted_runners: CaptureMut<'a, Vec<usize>, [usize]>,
    pub sans_self_runners: CaptureMut<'a, Vec<usize>, [usize]>,
    pub sticky_alloc: sticky_permuter::Alloc<'a>,
    pub summary: CaptureMut<'a, Matrix<f64>>,
}

impl<'a> Alloc<'a> {
    pub fn new(runners: usize, ranks: usize) -> Self {
        Self {
            podium: vec![0; ranks].into(),
            sorted_runners: vec![0; runners].into(),
            sans_self_runners: vec![0; runners - 1].into(),
            sticky_alloc: sticky_permuter::Alloc::new(ranks - 1),
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

pub fn summary_no_alloc(probs: &Matrix<f64>, degree: usize, alloc: &mut Alloc) {
    let Alloc {
        podium,
        sorted_runners,
        sans_self_runners,
        sticky_alloc,
        summary,
    } = alloc;
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
        sorted_runners.len(),
        probs.cols(),
        "length of sorted runners must equal the number of columns in the probabilities matrix"
    );
    debug_assert_eq!(
        probs.cols() - 1,
        sans_self_runners.len(),
        "sans-self runners length must be one less than the number of columns in the probabilities matrix"
    );
    debug_assert_eq!(
        sticky_alloc.ordinals.len(),
        probs.rows() - 1,
        "sticky permuter allocation must be be one less than the number of columns in the probabilities matrix"
    );

    let ranks = probs.rows();
    let runners = probs.cols();

    // initialise runner list for sorting
    for runner in 0..runners {
        sorted_runners[runner] = runner;
    }

    // sort runners in decreasing order of win probability
    sorted_runners.sort_unstable_by(|a, b| {
        let a_prob = probs[(0, *a)];
        let b_prob = probs[(0, *b)];
        b_prob.total_cmp(&a_prob)
    });

    let quota = count_permutations(runners - 1, degree - 1);
    for rank in 1..ranks {
        let podium = &mut podium[..rank + 1];

        for runner in 0..runners {
            for (ss_index, ss_runner) in sorted_runners
                .iter()
                .filter(|&&index| index != runner)
                .enumerate()
            {
                sans_self_runners[ss_index] = *ss_runner;
            }
            let total_permutations = count_permutations(runners - 1, rank);
            log::trace!(
                "runner: {runner}: rank: {rank}, total_perms: {total_permutations}, quota: {quota}, sans_self_runners: {sans_self_runners:?}"
            );

            // take slices of sticky_alloc to fit the needs of permuting over all outcomes for the current rank
            let sticky_alloc_slices = sticky_permuter::Alloc {
                combiner_ordinals: CaptureMut::Borrowed(
                    &mut sticky_alloc.combiner_ordinals[..rank],
                ),
                whole_ordinals_stack: CaptureMut::Borrowed(
                    &mut sticky_alloc.whole_ordinals_stack[..rank * (rank + 1) / 2],
                ),
                split_ordinals_stack: CaptureMut::Borrowed(
                    &mut sticky_alloc.split_ordinals_stack[..rank * (rank + 1) / 2],
                ),
                ordinals: CaptureMut::Borrowed(&mut sticky_alloc.ordinals[..rank]),
            };
            let mut permutation = 0;
            sticky_permuter::permute_no_alloc(runners - 1, rank, sticky_alloc_slices, |ordinals| {
                for (index, ordinal) in &mut ordinals.iter().enumerate() {
                    podium[index] = sans_self_runners[*ordinal];
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

    // assign the win probs from the source to the first row
    for (index, prob) in summary.row_slice_mut(0).iter_mut().enumerate() {
        *prob = probs[(0, index)]
    }

    // normalise remaining rows (2nd rank onwards)
    for row_idx in 1..summary.rows() {
        summary.row_slice_mut(row_idx).normalise(1.0);
    }
}

// pub fn summary(probs: &Matrix<f64>, ranks: usize, degree: usize) -> Matrix<f64> {
//     let runners = probs.cols();
//     let mut summary = Matrix::allocate(ranks, runners);
//     let mut podium = vec![0; ranks];
//     let mut bitmap = vec![false; runners - 1];
//     summary_no_alloc(
//         probs,
//         ranks,
//         degree,
//         &mut podium,
//         &mut bitmap,
//         &mut summary,
//     );
//     summary
// }
//
// pub fn summary_no_alloc(
//     probs: &Matrix<f64>,
//     ranks: usize,
//     degree: usize,
//     podium: &mut [usize],
//     bitmap: &mut [bool],
//     summary: &mut Matrix<f64>,
// ) {
//     debug_assert_eq!(
//         probs.rows(),
//         ranks,
//         "number of rows in the probabilities matrix must equal to the number of ranks"
//     );
//     debug_assert_eq!(
//         summary.rows(),
//         probs.rows(),
//         "number of rows in the probabilities matrix must equal to the number of rows in the summary matrix"
//     );
//     debug_assert_eq!(
//         summary.cols(),
//         probs.cols(),
//         "number of columns in the probabilities matrix must equal to the number of columns in the summary matrix"
//     );
//     debug_assert_eq!(
//         probs.rows(),
//         podium.len(),
//         "number of rows in the probabilities matrix must equal to the podium length"
//     );
//     debug_assert_eq!(
//         probs.cols() - 1,
//         bitmap.len(),
//         "bitmap length must be one less than the number of columns in the probabilities matrix"
//     );
//     let runners = probs.cols();
//
//     //TODO remove allocations
//     let mut sorted_runners = (0..runners).collect::<Vec<_>>();
//
//     //TODO can live with a smaller bitmap (n - 1 in size)
//
//     //TODO can also live with a smaller podium
//
//     // sort runners in decreasing order of win probability
//     sorted_runners.sort_unstable_by(|a, b| {
//         let a_prob = probs[(0, *a)];
//         let b_prob = probs[(0, *b)];
//         b_prob.total_cmp(&a_prob)
//     });
//
//     let quota = count_permutations(runners - 1, degree - 1);
//
//     for rank in 1..ranks {
//         let podium = &mut podium[..rank + 1];
//
//         for runner in 0..runners {
//             let sans_self = sorted_runners.iter().filter(|&&index| index != runner).collect::<Vec<_>>();
//             let total_permutations = count_permutations(runners - 1, rank);
//             log::trace!("runner: {runner}: rank: {rank}, total_perms: {total_permutations}, quota: {quota}, sans_self: {sans_self:?}");
//
//             let mut permutation = 0;
//             sticky_permuter::permute(runners - 1, rank, |ordinals| {
//                 for (index, ordinal) in &mut ordinals.iter().enumerate() {
//                     podium[index] = *sans_self[*ordinal];
//                 }
//                 podium[rank] = runner;
//                 let prob = harville(probs, podium);
//                 log::trace!("  podium: {podium:?}, prob: {prob:.6}");
//                 summary[(rank, runner)] += prob;
//
//                 permutation += 1;
//                 permutation < quota
//             });
//         }
//     }
//
//     // assign the win probs from the source
//     for (index, prob) in summary.row_slice_mut(0).iter_mut().enumerate() {
//         *prob = probs[(0, index)]
//     }
//
//     // normalise remaining rows (2nd rank onwards)
//     for row_idx in 1..summary.rows() {
//         summary.row_slice_mut(row_idx).normalise(1.0);
//     }
// }
