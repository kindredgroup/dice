use dice::each_way::probs_sim::{Scenario, Stats};
use dice::each_way::probs_sim;
use stanza::renderer::markdown::Markdown;
use stanza::renderer::Renderer;
use stanza::style::{HAlign, Header, Styles};
use stanza::table::{Col, Row, Table};
use tinyrand::StdRand;

const TRIALS: usize = 1_000;

fn main() {
    env_logger::init();

    let results = simulate_all(vec![
        Scenario { field: 8, k: 2 },
        Scenario { field: 8, k: 3 },
        Scenario { field: 8, k: 4 },
        Scenario { field: 8, k: 5 },
        Scenario { field: 8, k: 6 },
        Scenario { field: 8, k: 7 },
        Scenario { field: 10, k: 3 },
        Scenario { field: 10, k: 4 },
        Scenario { field: 10, k: 5 },
        Scenario { field: 10, k: 6 },
        Scenario { field: 10, k: 7 },
        Scenario { field: 10, k: 8 },
        Scenario { field: 10, k: 9 },
        Scenario { field: 12, k: 2 },
        Scenario { field: 12, k: 3 },
        Scenario { field: 12, k: 4 },
        Scenario { field: 12, k: 5 },
        Scenario { field: 12, k: 6 },
        Scenario { field: 12, k: 7 },
        Scenario { field: 12, k: 8 },
        Scenario { field: 18, k: 3 },
        Scenario { field: 18, k: 4 },
        Scenario { field: 18, k: 5 },
        Scenario { field: 18, k: 6 },
        Scenario { field: 20, k: 3 },
        Scenario { field: 20, k: 4 },
        Scenario { field: 20, k: 5 },
        Scenario { field: 20, k: 6},
        Scenario { field: 24, k: 3 },
        Scenario { field: 24, k: 4 },
        Scenario { field: 24, k: 5 },
        Scenario { field: 24, k: 6 },
        Scenario { field: 36, k: 4 },
        Scenario { field: 36, k: 5 },
        // Scenario { field: 36, k: 6 },
    ]);

    log::info!("Trials: {TRIALS}");
    let table = Table::default()
        .with_cols(
            (0..9)
                .map(|_| Col::new(Styles::default().with(HAlign::Right)))
                .collect(),
        )
        .with_row(Row::new(Styles::default().with(Header(true)), vec![
            "Field".into(),
            "Places".into(),
            "RMSRE mean".into(),
            "RMSE mean".into(),
            "RMSE p(.50)".into(),
            "RMSE p(.90)".into(),
            "RMSE p(.95)".into(),
            "RMSE p(.99)".into(),
            "RMSE p(1.0)".into(),
        ]))
        .with_rows(results.iter().map(|(scenario, stats)| {
            let quantile_errors = stats.quantiles(|errors| errors.rmse, &[0.5, 0.9, 0.95, 0.99, 1.0]);
            Row::new(Styles::default(), vec![
                format!("{}", scenario.field).into(),
                format!("{}", scenario.k).into(),
                format!("{:.6}", stats.mean.rmsre).into(),
                format!("{:.6}", stats.mean.rmse).into(),
                format!("{:.6}", quantile_errors[0]).into(),
                format!("{:.6}", quantile_errors[1]).into(),
                format!("{:.6}", quantile_errors[2]).into(),
                format!("{:.6}", quantile_errors[3]).into(),
                format!("{:.6}", quantile_errors[4]).into(),
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
                &dice::each_way::win_to_harville_place_probs,
                // &dice::each_way::win_to_baor_redist_place_probs,
                // &dice::each_way::win_to_est_place_probs,
                &|win_probs, k| {
                    dice::each_way::win_to_upscaled_place_probs(win_probs, k, std::cmp::min(k - 2, 2))
                }
                // &|win_probs, k| {
                //     dice::each_way::win_to_poly_harville_place_probs(win_probs, k, 4)
                // }
            );
            (scenario, stats)
        })
        .collect()
}
