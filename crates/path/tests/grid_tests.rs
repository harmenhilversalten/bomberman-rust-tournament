use path::grid::{PathGrid, movement_cost};
use path::{Grid, Point};

#[test]
fn grid_setup_and_costs() {
    let mut grid = PathGrid::new(3, 3);
    grid.set_walkable(Point::new(1, 1), false);
    grid.set_cost(Point::new(2, 2), 5);

    assert!(!grid.is_walkable(Point::new(1, 1)));
    assert_eq!(grid.influence(Point::new(2, 2)), 4);

    let neighbors = grid.neighbors(Point::new(1, 2));
    assert!(!neighbors.contains(&Point::new(1, 1)));

    let cost = movement_cost(&grid, Point::new(0, 0), Point::new(2, 2));
    assert_eq!(cost, 5);
}
