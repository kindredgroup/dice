use dice::probs::SliceExt;
use stanza::renderer::Renderer;
use stanza::renderer::markdown::Markdown;
use stanza::style::{HAlign, Header, Styles};
use stanza::table::{Col, Row, Table};
use tinyrand::{Rand, StdRand};
use dice::random;

const FIELD: usize = 12;
const RUNS: usize = 1000;

/// Scale parameter for the exponential probability allocator.
const BETA: f64 = 0.25;

fn main() {
    env_logger::init();

    let mut rand = StdRand::default();
    let all_probs = (0..RUNS)
        .map(|_| generate_random_probs(FIELD, &mut rand))
        .collect::<Vec<_>>();

    let table = Table::default()
        .with_cols(
            (0..FIELD)
                .map(|_| Col::new(Styles::default().with(HAlign::Right)))
                .collect(),
        )
        .with_row(Row::new(
            Styles::default().with(Header(true)),
            (0..FIELD).map(|i| format!("p{}", i + 1).into()).collect(),
        ))
        .with_rows(all_probs.iter().map(|probs| {
            Row::new(
                Styles::default(),
                probs
                    .iter()
                    .map(|prob| format!("{prob:.4}").into())
                    .collect(),
            )
        }));
    log::info!("Probs:\n{}", Markdown::default().render(&table));
    
    let mut avg_probs = (0..FIELD).map(|_| 0.0).collect::<Vec<_>>();
    for i in 0..FIELD {
        avg_probs[i] = all_probs.iter().map(|probs| probs[i]).sum();
    }
    for prob in &mut avg_probs {
        *prob /= RUNS as f64;
    }
    log::info!("expectation={avg_probs:?}");
    
    let avg_prices = avg_probs.invert().collect::<Vec<_>>();
    log::info!("avg_prices={avg_prices:?}");
}

fn generate_random_probs(field: usize, rand: &mut impl Rand) -> Vec<f64> {
    let mut probs = (0..field).map(|_| 0.0).collect::<Vec<_>>();
    probs.fill_random_probs_exp(rand, &random::gaussian_3_sigma, BETA, 1.0);
    probs
}
