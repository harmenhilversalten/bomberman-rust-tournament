use influence::core::{DangerSource, InfluenceMap, InfluenceType};
use influence::visualization::{export, renderer};
use state::GameState;

#[test]
fn ascii_renderer_outputs_grid() {
    let mut map = InfluenceMap::new(2, 2);
    map.add_danger_source(DangerSource {
        x: 0,
        y: 0,
        strength: 1.0,
        range: 2,
    });
    map.update(&GameState::new(2, 2)).unwrap();
    let out = renderer::render_ascii(&map, InfluenceType::Danger).unwrap();
    assert_eq!(out, "1.00 0.50\n0.50 0.00\n");
}

#[test]
fn csv_exporter_outputs_grid() {
    let mut map = InfluenceMap::new(2, 2);
    map.add_danger_source(DangerSource {
        x: 0,
        y: 0,
        strength: 1.0,
        range: 2,
    });
    map.update(&GameState::new(2, 2)).unwrap();
    let out = export::export_csv(&map, InfluenceType::Danger).unwrap();
    assert_eq!(out, "1.00,0.50\n0.50,0.00");
}
