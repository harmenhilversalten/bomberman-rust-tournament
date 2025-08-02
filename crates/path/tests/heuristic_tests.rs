use path::Point;
use path::heuristic::{Euclidean, Heuristic, Manhattan};

#[test]
fn manhattan_distance_is_correct() {
    let h = Manhattan;
    let a = Point::new(0, 0);
    let b = Point::new(3, 4);
    assert_eq!(h.distance(a, b), 7);
}

#[test]
fn euclidean_distance_is_correct() {
    let h = Euclidean;
    let a = Point::new(0, 0);
    let b = Point::new(3, 4);
    assert_eq!(h.distance(a, b), 5);
}
