use path::optimization::{simplify_path, smooth_path};
use path::{Grid, Point};

struct TestGrid {
    width: i32,
    height: i32,
    blocked: Vec<bool>,
}

impl TestGrid {
    fn new(width: i32, height: i32, blocked_cells: &[(i32, i32)]) -> Self {
        let mut blocked = vec![false; (width * height) as usize];
        for &(x, y) in blocked_cells {
            blocked[(y * width + x) as usize] = true;
        }
        Self {
            width,
            height,
            blocked,
        }
    }
}

impl Grid for TestGrid {
    fn width(&self) -> i32 {
        self.width
    }
    fn height(&self) -> i32 {
        self.height
    }
    fn is_walkable(&self, p: Point) -> bool {
        let idx = (p.y * self.width + p.x) as usize;
        !self.blocked[idx]
    }
}

#[test]
fn simplification_removes_collinear_points() {
    let path = vec![
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(2, 0),
        Point::new(2, 1),
    ];
    let simplified = simplify_path(&path);
    assert_eq!(
        simplified,
        vec![Point::new(0, 0), Point::new(2, 0), Point::new(2, 1)]
    );
}

#[test]
fn smoothing_skips_detours() {
    let grid = TestGrid::new(3, 2, &[(1, 0)]); // block (1,0) to force detour
    let path = vec![
        Point::new(0, 0),
        Point::new(0, 1),
        Point::new(1, 1),
        Point::new(2, 1),
    ];
    let smoothed = smooth_path(&grid, &path);
    assert_eq!(
        smoothed,
        vec![Point::new(0, 0), Point::new(0, 1), Point::new(2, 1)]
    );
}
