use path::algorithms::{AStar, DStarLite, JumpPointSearch, Pathfinder};
use path::{Grid, Point};

struct TestGrid {
    width: i32,
    height: i32,
    blocked: Vec<bool>,
    influence: Vec<i32>,
}

impl TestGrid {
    fn new(width: i32, height: i32, blocked_cells: &[(i32, i32)]) -> Self {
        let mut blocked = vec![false; (width * height) as usize];
        for &(x, y) in blocked_cells {
            let idx = (y * width + x) as usize;
            blocked[idx] = true;
        }
        Self {
            width,
            height,
            blocked,
            influence: vec![0; (width * height) as usize],
        }
    }

    fn set_influence(&mut self, pos: Point, value: i32) {
        let idx = (pos.y * self.width + pos.x) as usize;
        self.influence[idx] = value;
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

    fn influence(&self, p: Point) -> i32 {
        let idx = (p.y * self.width + p.x) as usize;
        self.influence[idx]
    }
}

fn run_all_algorithms(grid: &TestGrid, start: Point, goal: Point) -> Vec<Vec<Point>> {
    let mut astar = AStar::new();
    let mut dstar = DStarLite::new();
    let mut jps = JumpPointSearch::new();

    vec![
        astar.find_path(grid, start, goal).unwrap(),
        dstar.find_path(grid, start, goal).unwrap(),
        jps.find_path(grid, start, goal).unwrap(),
    ]
}

fn verify_path(grid: &TestGrid, path: &[Point], start: Point, goal: Point) {
    assert_eq!(path.first(), Some(&start));
    assert_eq!(path.last(), Some(&goal));
    for window in path.windows(2) {
        let a = window[0];
        let b = window[1];
        let dx = (a.x - b.x).abs();
        let dy = (a.y - b.y).abs();
        assert!(dx + dy == 1, "points are not adjacent");
        assert!(grid.is_walkable(b));
    }
}

#[test]
fn algorithms_find_valid_path() {
    // 5x5 grid with obstacles
    let blocked = vec![(3, 0), (1, 1), (3, 1), (1, 2), (1, 3), (2, 3), (3, 3)];
    let grid = TestGrid::new(5, 5, &blocked);
    let start = Point::new(0, 0);
    let goal = Point::new(4, 4);

    let paths = run_all_algorithms(&grid, start, goal);
    for path in paths {
        verify_path(&grid, &path, start, goal);
    }
}

#[test]
fn influence_penalty_is_respected() {
    let mut grid = TestGrid::new(3, 3, &[]);
    grid.set_influence(Point::new(1, 1), 100); // high penalty in center
    let start = Point::new(0, 1);
    let goal = Point::new(2, 1);

    let paths = run_all_algorithms(&grid, start, goal);
    for path in paths {
        // The optimal path should avoid the center tile (1,1)
        assert!(!path.contains(&Point::new(1, 1)));
    }
}
