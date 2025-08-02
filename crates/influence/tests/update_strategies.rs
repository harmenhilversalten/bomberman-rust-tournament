use influence::{DangerSource, FullUpdate, InfluenceMap};
use proptest::prelude::*;
use state::GameState;

proptest! {
    #[test]
    fn full_matches_incremental(
        width in 1u16..8,
        height in 1u16..8,
        x in 0u16..8,
        y in 0u16..8,
        range in 1u16..4,
        strength in 0.0f32..5.0
    ) {
        let width = width.max(1);
        let height = height.max(1);
        let x = x % width;
        let y = y % height;
        let range = range.min(width.max(height));
        let source = DangerSource { x, y, strength, range };
        let state = GameState::new(width as usize, height as usize);

        let mut full = InfluenceMap::with_strategy(width, height, Box::new(FullUpdate::new()));
        let mut inc = InfluenceMap::new(width, height);

        full.add_danger_source(source);
        inc.add_danger_source(source);

        full.update(&state).unwrap();
        inc.update(&state).unwrap();

        for yy in 0..height {
            for xx in 0..width {
                prop_assert!((full.danger_at(xx, yy).unwrap() - inc.danger_at(xx, yy).unwrap()).abs() < 1e-6);
            }
        }
    }
}
