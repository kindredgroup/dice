use dice::each_way::probs_sim::{Scenario, Stats};
use dice::each_way::{probs_sim, win_to_harville_place_probs, win_to_opt_place_probs};
use stanza::renderer::Renderer;
use stanza::renderer::markdown::Markdown;
use stanza::style::{HAlign, Header, Styles};
use stanza::table::{Col, Row, Table};
use tinyrand::StdRand;

const TRIALS: usize = 1_000;

fn main() {
    env_logger::init();

    let results = simulate_all(vec![
        // Scenario { field: 8, k: 2 },
        // Scenario { field: 12, k: 2 },
        // Scenario { field: 12, k: 3 },
        // Scenario { field: 18, k: 3 },
        // Scenario { field: 18, k: 4 },
        // Scenario { field: 20, k: 3 },
        // Scenario { field: 20, k: 4 },
        // Scenario { field: 20, k: 5 },
        Scenario { field: 20, k: 6},
        Scenario { field: 20, k: 7},
        // Scenario { field: 24, k: 3 },
        // Scenario { field: 24, k: 4 },
        // Scenario { field: 24, k: 5 },
        // Scenario { field: 36, k: 4 },
        // Scenario { field: 36, k: 5 },
        // Scenario { field: 36, k: 6 },
    ]);

    log::info!("Trials: {TRIALS}");
    let table = Table::default()
        .with_cols(
            (0..4)
                .map(|_| Col::new(Styles::default().with(HAlign::Right)))
                .collect(),
        )
        .with_row(Row::new(Styles::default().with(Header(true)), vec![
            "Field".into(),
            "Places".into(),
            "RMSE".into(),
            "RMSRE".into(),
        ]))
        .with_rows(results.iter().map(|(scenario, stats)| {
            Row::new(Styles::default(), vec![
                format!("{}", scenario.field).into(),
                format!("{}", scenario.k).into(),
                format!("{:.6}", stats.rmse).into(),
                format!("{:.6}", stats.rmsre).into(),
            ])
        }));
    log::info!("Summary:\n{}", Markdown::default().render(&table));
}

fn simulate_all(scenarios: Vec<Scenario>) -> Vec<(Scenario, Stats)> {
    let mut rand = StdRand::default();
    scenarios
        .into_iter()
        .map(|scenario| {
            let stats = probs_sim::simulate(
                &scenario,
                TRIALS,
                &mut rand,
                &win_to_harville_place_probs,
                &|win_probs, k| {
                    win_to_opt_place_probs(win_probs, k, std::cmp::min(k - 2, 2))
                }
            );
            (scenario, stats)
        })
        .collect()
}
