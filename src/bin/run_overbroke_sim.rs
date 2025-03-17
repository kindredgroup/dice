use dice::each_way::overbroke_sim;
use dice::each_way::overbroke_sim::{Estimator, Scenario, Stats};
use stanza::renderer::Renderer;
use stanza::renderer::markdown::Markdown;
use stanza::style::{HAlign, Header, Styles};
use stanza::table::{Col, Row, Table};
use tinyrand::StdRand;

const TRIALS: usize = 1_000;

fn main() {
    env_logger::init();

    let results = simulate_all(vec![
        Scenario {
            field: 12,
            win_overround: 1.10,
            k: 2,
            d: 2,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 12,
            win_overround: 1.10,
            k: 2,
            d: 3,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 12,
            win_overround: 1.10,
            k: 2,
            d: 4,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 12,
            win_overround: 1.15,
            k: 2,
            d: 2,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 12,
            win_overround: 1.15,
            k: 2,
            d: 3,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 12,
            win_overround: 1.15,
            k: 2,
            d: 4,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 12,
            win_overround: 1.20,
            k: 2,
            d: 2,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 12,
            win_overround: 1.20,
            k: 2,
            d: 3,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 12,
            win_overround: 1.20,
            k: 2,
            d: 4,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 12,
            win_overround: 1.25,
            k: 2,
            d: 2,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 12,
            win_overround: 1.25,
            k: 2,
            d: 3,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 12,
            win_overround: 1.25,
            k: 2,
            d: 4,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 12,
            win_overround: 1.30,
            k: 2,
            d: 2,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 12,
            win_overround: 1.30,
            k: 2,
            d: 3,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 12,
            win_overround: 1.30,
            k: 2,
            d: 4,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 12,
            win_overround: 1.35,
            k: 2,
            d: 2,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 12,
            win_overround: 1.35,
            k: 2,
            d: 3,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 12,
            win_overround: 1.35,
            k: 2,
            d: 4,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 25,
            win_overround: 1.10,
            k: 3,
            d: 3,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 25,
            win_overround: 1.10,
            k: 3,
            d: 4,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 25,
            win_overround: 1.10,
            k: 3,
            d: 5,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 25,
            win_overround: 1.15,
            k: 3,
            d: 3,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 25,
            win_overround: 1.15,
            k: 3,
            d: 4,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 25,
            win_overround: 1.15,
            k: 3,
            d: 4,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 25,
            win_overround: 1.20,
            k: 3,
            d: 3,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 25,
            win_overround: 1.20,
            k: 3,
            d: 4,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 25,
            win_overround: 1.20,
            k: 3,
            d: 5,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 25,
            win_overround: 1.25,
            k: 3,
            d: 3,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 25,
            win_overround: 1.25,
            k: 3,
            d: 4,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 25,
            win_overround: 1.25,
            k: 3,
            d: 5,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 25,
            win_overround: 1.30,
            k: 3,
            d: 3,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 25,
            win_overround: 1.30,
            k: 3,
            d: 4,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 25,
            win_overround: 1.30,
            k: 3,
            d: 5,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 25,
            win_overround: 1.35,
            k: 3,
            d: 3,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 25,
            win_overround: 1.35,
            k: 3,
            d: 4,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 25,
            win_overround: 1.35,
            k: 3,
            d: 5,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 25,
            win_overround: 1.40,
            k: 3,
            d: 3,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 25,
            win_overround: 1.40,
            k: 3,
            d: 4,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 25,
            win_overround: 1.40,
            k: 3,
            d: 5,
            target_place_overround: 1.10,
            estimator: Estimator::Harville
        },
        Scenario {
            field: 50,
            win_overround: 1.10,
            k: 5,
            d: 5,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.10,
            k: 5,
            d: 6,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.10,
            k: 5,
            d: 7,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.10,
            k: 5,
            d: 8,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.15,
            k: 5,
            d: 5,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.15,
            k: 5,
            d: 6,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.15,
            k: 5,
            d: 7,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.15,
            k: 5,
            d: 8,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.20,
            k: 5,
            d: 5,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.20,
            k: 5,
            d: 6,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.20,
            k: 5,
            d: 7,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.20,
            k: 5,
            d: 8,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.25,
            k: 5,
            d: 5,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.25,
            k: 5,
            d: 6,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.25,
            k: 5,
            d: 7,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.25,
            k: 5,
            d: 8,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.30,
            k: 5,
            d: 5,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.30,
            k: 5,
            d: 6,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.30,
            k: 5,
            d: 7,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.30,
            k: 5,
            d: 8,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.35,
            k: 5,
            d: 5,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.35,
            k: 5,
            d: 6,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.35,
            k: 5,
            d: 7,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.35,
            k: 5,
            d: 8,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.40,
            k: 5,
            d: 5,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.40,
            k: 5,
            d: 6,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.40,
            k: 5,
            d: 7,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.40,
            k: 5,
            d: 8,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.45,
            k: 5,
            d: 5,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.45,
            k: 5,
            d: 6,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.45,
            k: 5,
            d: 7,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.45,
            k: 5,
            d: 8,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.50,
            k: 5,
            d: 5,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.50,
            k: 5,
            d: 6,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.50,
            k: 5,
            d: 7,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.50,
            k: 5,
            d: 8,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.55,
            k: 5,
            d: 5,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.55,
            k: 5,
            d: 6,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.55,
            k: 5,
            d: 7,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.55,
            k: 5,
            d: 8,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.60,
            k: 5,
            d: 5,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.60,
            k: 5,
            d: 6,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.60,
            k: 5,
            d: 7,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 50,
            win_overround: 1.60,
            k: 5,
            d: 8,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.20,
            k: 7,
            d: 7,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.20,
            k: 7,
            d: 8,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.20,
            k: 7,
            d: 9,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.20,
            k: 7,
            d: 10,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.25,
            k: 7,
            d: 7,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.25,
            k: 7,
            d: 8,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.25,
            k: 7,
            d: 9,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.25,
            k: 7,
            d: 10,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.30,
            k: 7,
            d: 7,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.30,
            k: 7,
            d: 8,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.30,
            k: 7,
            d: 9,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.30,
            k: 7,
            d: 10,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.35,
            k: 7,
            d: 7,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.35,
            k: 7,
            d: 8,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.35,
            k: 7,
            d: 9,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.35,
            k: 7,
            d: 10,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.40,
            k: 7,
            d: 7,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.40,
            k: 7,
            d: 8,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.40,
            k: 7,
            d: 9,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.40,
            k: 7,
            d: 10,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.45,
            k: 7,
            d: 7,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.45,
            k: 7,
            d: 8,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.45,
            k: 7,
            d: 9,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.45,
            k: 7,
            d: 10,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.50,
            k: 7,
            d: 7,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.50,
            k: 7,
            d: 8,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.50,
            k: 7,
            d: 9,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.50,
            k: 7,
            d: 10,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.55,
            k: 7,
            d: 7,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.55,
            k: 7,
            d: 8,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.55,
            k: 7,
            d: 9,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.55,
            k: 7,
            d: 10,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.60,
            k: 7,
            d: 7,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.60,
            k: 7,
            d: 8,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.60,
            k: 7,
            d: 9,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.60,
            k: 7,
            d: 10,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.65,
            k: 7,
            d: 7,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.65,
            k: 7,
            d: 8,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.65,
            k: 7,
            d: 9,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.65,
            k: 7,
            d: 10,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        },
        Scenario {
            field: 100,
            win_overround: 1.65,
            k: 7,
            d: 10,
            target_place_overround: 1.10,
            estimator: Estimator::Upscaling(2)
        }
    ]);

    log::info!("Trials: {TRIALS}");
    let table = Table::default()
        .with_cols(
            (0..11)
                .map(|_| Col::new(Styles::default().with(HAlign::Right)))
                .collect(),
        )
        .with_row(Row::new(Styles::default().with(Header(true)), vec![
            "Field".into(),
            "Win o/r".into(),
            "Places".into(),
            "Split".into(),
            "Target place o/r".into(),
            "Average place o/r".into(),
            "Overbroke %".into(),
            "Under target booksum %".into(),
            "At least one value outcome %".into(),
            "Value outcomes per field %".into(),
            "Estimator".into(),
        ]))
        .with_rows(results.iter().map(|(scenario, stats)| {
            Row::new(Styles::default(), vec![
                format!("{}", scenario.field).into(),
                format!("{:.2}", scenario.win_overround).into(),
                format!("{}", scenario.k).into(),
                format!("{}", scenario.d).into(),
                format!("{:.2}", scenario.target_place_overround).into(),
                format!("{:.2}", stats.average_place_overround).into(),
                format!(
                    "{:.2}",
                    stats.total_overbroke as f64 / TRIALS as f64 * 100.0
                )
                .into(),
                format!(
                    "{:.2}",
                    stats.total_under_target_booksum as f64 / TRIALS as f64 * 100.0
                )
                .into(),
                format!(
                    "{:.2}",
                    stats.total_at_least_one_value_outcome as f64 / TRIALS as f64 * 100.0
                )
                .into(),
                format!(
                    "{:.2}",
                    stats.total_value_outcomes as f64 / TRIALS as f64 / scenario.field as f64
                        * 100.0
                )
                .into(),
                format!(
                    "{:?}",
                    scenario.estimator
                )
                .into(),
            ])
        }));
    log::info!("Summary:\n{}", Markdown::default().render(&table));
}

fn simulate_all(scenarios: Vec<Scenario>) -> Vec<(Scenario, Stats)> {
    let mut rand = StdRand::default();
    scenarios
        .into_iter()
        .map(|scenario| {
            let stats = overbroke_sim::simulate(&scenario, TRIALS, &mut rand);
            (scenario, stats)
        })
        .collect()
}
