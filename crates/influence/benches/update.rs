use criterion::{Criterion, black_box, criterion_group, criterion_main};
use influence::core::{DangerSource, InfluenceMap, OpportunitySource};
use state::GameState;

fn update_benchmark(c: &mut Criterion) {
    let mut map = InfluenceMap::new(32, 32);
    for i in 0..8 {
        map.add_danger_source(DangerSource {
            x: i * 3,
            y: i * 3,
            strength: 1.0,
            range: 4,
        });
        map.add_opportunity_source(OpportunitySource {
            x: 31 - i * 3,
            y: i * 3,
            value: 1.0,
            range: 4,
        });
    }
    let state = GameState::new(32, 32);
    let memory = (map.width() as usize * map.height() as usize * std::mem::size_of::<f32>()) * 2;
    println!("approx memory usage: {} bytes", memory);
    c.bench_function("influence_update", |b| {
        b.iter(|| {
            map.update(black_box(&state)).unwrap();
        });
    });
}

criterion_group!(benches, update_benchmark);
criterion_main!(benches);
