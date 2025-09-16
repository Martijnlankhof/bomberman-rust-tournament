use criterion::{criterion_group, criterion_main, Criterion};
use rust_bomberman::{tournament, bot::available_bots};
use rust_bomberman::tournament::prepare_bots;

fn tournament_benchmark(c: &mut Criterion) {
    let mut config = Criterion::default()
        .sample_size(100) // default is 100; increase for more precision
        .measurement_time(std::time::Duration::from_secs(120)) // default is 5s
        .warm_up_time(std::time::Duration::from_secs(5)); // default is 3s


    config.bench_function("run_tournament", |b| {
        b.iter(|| {
            let bots = available_bots();
            let game_bots = prepare_bots(&bots);


            tournament::run_game(game_bots);
        });
    });

}

criterion_group!(benches, tournament_benchmark);
criterion_main!(benches);
