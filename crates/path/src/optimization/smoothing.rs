use crate::{Grid, Point};

/// Removes unnecessary waypoints when a straight line is available.
pub fn smooth_path<G: Grid>(grid: &G, path: &[Point]) -> Vec<Point> {
    if path.len() <= 2 {
        return path.to_vec();
    }
    let mut result = Vec::with_capacity(path.len());
    let mut i = 0;
    while i < path.len() - 1 {
        result.push(path[i]);
        let mut j = i + 1;
        for k in (i + 1)..path.len() {
            if is_clear(grid, path[i], path[k]) {
                j = k;
            } else {
                break;
            }
        }
        i = j;
    }
    if *result.last().unwrap() != *path.last().unwrap() {
        result.push(*path.last().unwrap());
    }
    result
}

fn is_clear<G: Grid>(grid: &G, a: Point, b: Point) -> bool {
    if a.x == b.x {
        let (start, end) = if a.y <= b.y { (a.y, b.y) } else { (b.y, a.y) };
        for y in start..=end {
            if !grid.is_walkable(Point::new(a.x, y)) {
                return false;
            }
        }
        true
    } else if a.y == b.y {
        let (start, end) = if a.x <= b.x { (a.x, b.x) } else { (b.x, a.x) };
        for x in start..=end {
            if !grid.is_walkable(Point::new(x, a.y)) {
                return false;
            }
        }
        true
    } else {
        false
    }
}
