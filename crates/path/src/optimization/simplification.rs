use crate::Point;

/// Removes intermediate points that are collinear.
pub fn simplify_path(path: &[Point]) -> Vec<Point> {
    if path.len() <= 2 {
        return path.to_vec();
    }
    let mut result = Vec::with_capacity(path.len());
    result.push(path[0]);
    for window in path.windows(3) {
        let a = window[0];
        let b = window[1];
        let c = window[2];
        let ab = (b.x - a.x, b.y - a.y);
        let bc = (c.x - b.x, c.y - b.y);
        if ab.0 * bc.1 != ab.1 * bc.0 {
            result.push(b);
        }
    }
    result.push(*path.last().unwrap());
    result
}
