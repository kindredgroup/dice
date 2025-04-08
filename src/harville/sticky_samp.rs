use crate::comb::{count_permutations, sticky};
use crate::harville::harville;
use crate::matrix::Matrix;
use crate::probs::SliceExt;

pub fn summary(probs: &Matrix<f64>, ranks: usize, degree: usize) -> Matrix<f64> {
    let runners = probs.cols();
    let mut summary = Matrix::allocate(ranks, runners);
    let mut podium = vec![0; ranks];
    let mut bitmap = vec![false; runners - 1];
    summary_no_alloc(
        probs,
        ranks,
        degree,
        &mut podium,
        &mut bitmap,
        &mut summary,
    );
    summary
}

pub fn summary_no_alloc(
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