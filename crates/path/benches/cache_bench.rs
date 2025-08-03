use criterion::{Criterion, black_box, criterion_group, criterion_main};
use path::algorithms::Pathfinder;
use path::cache::{CacheKey, EvictionPolicy, PathCache};
use path::{AStar, PathGrid, Point};

fn bench_astar_with_cache(c: &mut Criterion) {
    let grid = PathGrid::new(10, 10);
    let mut astar = AStar::new();
    let start = Point::new(0, 0);
    let goal = Point::new(9, 9);

    c.bench_function("astar_no_cache", |b| {
        b.iter(|| {
            black_box(astar.find_path(&grid, start, goal).unwrap());
        })
    });

    c.bench_function("astar_with_cache", |b| {
        let mut cache = PathCache::new(16, EvictionPolicy::Lru);
        b.iter(|| {
            let key = CacheKey::new(start, goal);
            if let Some(p) = cache.get(&key) {
                black_box(p.len());
            } else {
                let path = astar.find_path(&grid, start, goal).unwrap();
                cache.insert(key, path);
            }
        })
    });
}

criterion_group!(benches, bench_astar_with_cache);
criterion_main!(benches);
