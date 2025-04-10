use criterion::{criterion_group, criterion_main, Criterion};
use dice::capture::Capture;
use dice::dilative::DilatedProbs;
use dice::harville::rand_samp;
use dice::matrix::Matrix;
use dice::probs::SliceExt;
use dice::random;
use std::time::Duration;
use tinyrand::StdRand;

fn criterion_benchmark(c: &mut Criterion) {
    fn bench(c: &mut Criterion, n: usize, k: usize, degree: usize) {
        let mut rand = StdRand::default();
        let mut win_probs = vec![0.0; n];
        win_probs.fill_random_probs_exp(&mut rand, &random::gaussian_3_sigma, 0.25, 1.0);
        let dilated_probs = Matrix::from(
            DilatedProbs::default()
                .with_win_probs(Capture::Owned(win_probs))
                .with_podium_places(k),
        );
        let mut alloc = rand_samp::Alloc::new(n, k);
        
        c.bench_function(&format!("cri_harville_rand_samp{n}x{k}_d{degree}"), |b| {
            b.iter(|| {
                rand_samp::summary_no_alloc(&dilated_probs, degree, &mut alloc);
            });
        });
    }
    bench(c, 6, 6, 3);
    bench(c, 6, 6, 4);
    bench(c, 6, 6, 5);
    bench(c, 7, 7, 4);
    bench(c, 7, 7, 5);
    bench(c, 7, 7, 6);
    bench(c, 8, 8, 4);
    bench(c, 8, 8, 5);
    bench(c, 8, 8, 6);
    bench(c, 9, 9, 4);
    bench(c, 9, 9, 5);
    bench(c, 9, 9, 6);
    bench(c, 10, 10, 4);
    bench(c, 10, 10, 5);
    bench(c, 10, 10, 6);
    bench(c, 11, 11, 4);
    bench(c, 11, 11, 5);
    bench(c, 11, 11, 6);
    bench(c, 12, 12, 4);
    bench(c, 13, 13, 4);
    bench(c, 14, 14, 4);
    bench(c, 15, 15, 4);
    bench(c, 16, 16, 4);
    bench(c, 17, 17, 4);
    bench(c, 18, 18, 4);
    bench(c, 19, 19, 4);
    bench(c, 20, 20, 4);
}

criterion_group!(name = benches; config = Criterion::default().measurement_time(Duration::from_secs(10)); targets = criterion_benchmark);
criterion_main!(benches);
