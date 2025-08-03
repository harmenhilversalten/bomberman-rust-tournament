use criterion::{criterion_group, criterion_main, Criterion};
use engine::{config::EngineConfig, Engine};

fn game_loop_performance(c: &mut Criterion) {
    c.bench_function("engine_tick", |b| {
        b.iter(|| {
            let (mut engine, _rx, _events) = Engine::new(EngineConfig::default());
            engine.tick().unwrap();
        })
    });
}

criterion_group!(benches, game_loop_performance);
criterion_main!(benches);
