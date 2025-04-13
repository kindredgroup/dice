use criterion::{criterion_group, criterion_main, Criterion};
use dice::capture::CaptureMut;
use dice::comb::combiner;
use dice::stream::generator::Generator;

fn criterion_benchmark(c: &mut Criterion) {
    fn bench(c: &mut Criterion, n: usize, r: usize) {
        let mut ordinals = combiner::Combiner::alloc(r);
        c.bench_function(&format!("cri_combiner_{n}c{r}"), |b| {
            b.iter(|| {
                let mut combiner = combiner::Combiner::new_no_alloc(n, CaptureMut::Borrowed(&mut ordinals));
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
    bench(c, 4, 2);
    bench(c, 4, 3);
    bench(c, 8, 4);
    bench(c, 8, 5);
    bench(c, 8, 6);
    bench(c, 8, 7);
    bench(c, 16, 8);
    bench(c, 16, 9);
    bench(c, 16, 10);
    bench(c, 16, 11);
    bench(c, 16, 12);
    bench(c, 16, 13);
    bench(c, 16, 14);
    bench(c, 16, 15);
    bench(c, 32, 31);
    bench(c, 64, 63);
    bench(c, 128, 127);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
