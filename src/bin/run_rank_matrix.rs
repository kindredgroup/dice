use dice::capture::Capture;
use dice::dilative::DilatedProbs;
use dice::harville::{poly_harville_summary, harville_summary};
use dice::matrix::Matrix;
use dice::probs::SliceExt;
use stanza::renderer::markdown::Markdown;
use stanza::renderer::Renderer;
use stanza::style::{Header, Styles};
use stanza::table::{Row, Table};

fn main() {
    env_logger::init();

    let win_probs = vec![0.2784197303099966, 0.1954372168433092, 0.14613262725141385, 0.10771732864366414, 0.0797980517422571, 0.058430296967786594, 0.04374205825964218, 0.030709575930595815, 0.022217682914963545, 0.016423615257347497, 0.012087131057607623, 0.008884684821415677];
    let k = win_probs.len();
    
    {
        let table = Table::default()
            .with_row(Row::new(
                Styles::default().with(Header(true)),
                (1..=win_probs.len())
                    .map(|i| format!("{i}").into())
                    .collect(),
            ))
            .with_row(Row::new(
                Styles::default(),
                win_probs
                    .iter()
                    .map(|prob| format!("{prob:.6}").into())
                    .collect(),
            ));
        log::info!("Win probs:\n{}", Markdown::default().render(&table));
    }

    let rank_probs = poly_harville(&win_probs, k);
    {
        let table = Table::default()
            .with_row(Row::new(
                Styles::default().with(Header(true)),
                (1..=win_probs.len())
                    .map(|i| format!("{i}").into())
                    .collect(),
            ))
            .with_rows(rank_probs.into_iter().map(|probs| {
                Row::new(
                    Styles::default(),
                    probs
                        .iter()
                        .map(|prob| format!("{prob:.6}").into())
                        .collect(),
                )
            }));
        log::info!("Rank matrix:\n{}", Markdown::default().render(&table));
    }
    
    let row_sums = rank_probs.into_iter().map(|probs| probs.sum()).collect::<Vec<_>>();
    {
        let table = Table::default()
            .with_row(Row::new(
                Styles::default().with(Header(true)),
                (1..=k)
                    .map(|i| format!("{i}").into())
                    .collect(),
            ))
            .with_row(Row::new(
                Styles::default(),
                row_sums
                    .iter()
                    .map(|row_sum| format!("{row_sum:.6}").into())
                    .collect(),
            ));
        log::info!("Row sums:\n{}", Markdown::default().render(&table));
    }

    let col_sums = rank_probs.transpose().into_iter().map(|probs| probs.sum()).collect::<Vec<_>>();
    {
        let table = Table::default()
            .with_row(Row::new(
                Styles::default().with(Header(true)),
                (1..=k)
                    .map(|i| format!("{i}").into())
                    .collect(),
            ))
            .with_row(Row::new(
                Styles::default(),
                col_sums
                    .iter()
                    .map(|col_sum| format!("{col_sum:.6}").into())
                    .collect(),
            ));
        log::info!("Col sums:\n{}", Markdown::default().render(&table));
    }
}

pub fn harville(win_probs: &[f64], k: usize) -> Matrix<f64> {
    let dilated_probs = Matrix::from(
        DilatedProbs::default()
            .with_win_probs(Capture::Borrowed(win_probs))
            .with_podium_places(k),
    );
    harville_summary(&dilated_probs, k)
}

pub fn poly_harville(win_probs: &[f64], k: usize) -> Matrix<f64> {
    const DEGREE: usize = 4;
    let dilated_probs = Matrix::from(
        DilatedProbs::default()
            .with_win_probs(Capture::Borrowed(win_probs))
            .with_podium_places(k),
    );
    poly_harville_summary(&dilated_probs, k, DEGREE)
}
