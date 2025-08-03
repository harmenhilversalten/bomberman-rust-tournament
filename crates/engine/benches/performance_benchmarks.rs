use criterion::{Criterion, criterion_group, criterion_main};
use engine::{Engine, config::EngineConfig};

fn engine_tick_benchmark(c: &mut Criterion) {
    c.bench_function("engine_tick", |b| {
        b.iter(|| {
            let (mut engine, _rx, _events) = Engine::new(EngineConfig::default());
            engine.tick().unwrap();
        });
    });
}

criterion_group!(benches, engine_tick_benchmark);
criterion_main!(benches);
