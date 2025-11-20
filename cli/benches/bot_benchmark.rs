use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use bots::{available_bots, get_bot_names};
use tournament::{factories::game_config_factory::odd_numbers_in_range, tournament::run_game};


fn bot_benchmark(c: &mut Criterion) {
    let bot_constructors = available_bots();
    let bot_names = get_bot_names();
    let map_sizes = odd_numbers_in_range(7, 20);

    for (index, bot_name) in bot_names.iter().enumerate() {
        c.bench_function(bot_name, |b| {
            b.iter(|| {
                for &map_size in &map_sizes {
                    for opponent_index in 0..bot_constructors.len() {
                        if index == opponent_index {
                            continue;
                        }

                        let bot = (bot_constructors[index])();
                        let opponent = (bot_constructors[opponent_index])();

                        let game_result = run_game(vec![bot, opponent], map_size);

                        black_box(&game_result);
                    }
                }
            })
        });
    }
}

fn custom_criterion() -> Criterion {
    Criterion::default()
        .warm_up_time(std::time::Duration::from_secs(5))
        .measurement_time(std::time::Duration::from_secs(120))
        .sample_size(200) 
        .configure_from_args()
}

criterion_group!{
    name = benches;
    config = custom_criterion();
    targets = bot_benchmark
}
criterion_main!(benches);
