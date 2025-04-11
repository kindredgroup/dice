use criterion::{criterion_group, criterion_main, Criterion};
use dice::capture::CaptureMut;
use dice::comb::split_combiner;

fn criterion_benchmark(c: &mut Criterion) {
    fn bench(c: &mut Criterion, n: usize) {
        let mut ordinals = split_combiner::SplitCombiner::alloc(n);
        c.bench_function(&format!("cri_split_combiner_{n}"), |b| {
            b.iter(|| {
                let mut combiner = split_combiner::SplitCombiner::new_no_alloc(CaptureMut::Borrowed(&mut ordinals));
                let mut iterations = 0;
                loop {
                    iterations += 1;
                    if !combiner.advance() {
                        break;
                    }
                }
                iterations
            });
        });
    }
    bench(c, 4);
    bench(c, 8);
    bench(c, 16);
    bench(c, 32);
    bench(c, 64);
    bench(c, 128);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
