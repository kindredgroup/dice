use criterion::{criterion_group, criterion_main, Criterion};
use tinyrand::StdRand;
use dice::capture::Capture;
use dice::dilative::DilatedProbs;
use dice::harville::harville_summary_no_alloc;
use dice::matrix::Matrix;
use dice::probs::SliceExt;
use dice::random;

fn criterion_benchmark(c: &mut Criterion) {
    fn bench(c: &mut Criterion, n: usize, k: usize) {
        let mut rand = StdRand::default();
        let mut win_probs = vec![0.0; n];
        win_probs.fill_random_probs_exp(&mut rand, &random::gaussian_3_sigma, 0.25, 1.0);
        let dilated_probs = Matrix::from(
            DilatedProbs::default()
                .with_win_probs(Capture::Owned(win_probs))
                .with_podium_places(k),
        );
        let mut summary = Matrix::allocate(k, n);
        let cardinalities = vec![n; k];
        let mut podium = vec![0; k];
        let mut bitmap = vec![false; n];
        
        c.bench_function(&format!("cri_harville_{n}x{k}"), |b| {
            b.iter(|| {
                harville_summary_no_alloc(&dilated_probs, k, &cardinalities, &mut podium, &mut bitmap, &mut summary);
            });
        });
    }
    bench(c, 3, 3);
    bench(c, 4, 4);
    bench(c, 5, 5);
    bench(c, 6, 6);
    bench(c, 7, 7);
    bench(c, 8, 8);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
