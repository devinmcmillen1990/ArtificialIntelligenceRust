mod pathfinding;
use pathfinding::grid_generator::generate_test_grid;
use utils::pathfinding::a_star::a_star;

#[test]
fn test_astar_no_obstacles() {
    let grid = generate_test_grid((5, 5), vec![]);
    let start = (0, 0);
    let goal = (4, 4);

    let path = a_star(&grid, start, goal);

    assert!(!path.is_empty(), "A* should find a path.");
    assert_eq!(path.first(), Some(&start), "Path should start at the start position.");
    assert_eq!(path.last(), Some(&goal), "Path should end at the goal position.");
}

#[test]
fn test_a_star_with_cycles() {
    // Arrange: A grid with cycles where some paths loop back to earlier points.
    let mut grid = generate_test_grid((5, 5), vec![]);
    grid[1][1] = true;
    grid[1][2] = true;
    grid[1][3] = true;
    grid[2][1] = true;
    grid[3][1] = true;
    grid[3][2] = true;
    grid[3][3] = true;

    let start = (0, 0);
    let goal = (4, 4);

    // Act: Perform A*.
    let path = a_star(&grid, start, goal);

    // Assert: Verify that A* avoids cycles and finds a valid path.
    assert!(!path.is_empty(), "A* should find a valid path.");
    assert_eq!(path.first(), Some(&start), "Path should start at the start position.");
    assert_eq!(path.last(), Some(&goal), "Path should end at the goal position.");
}

#[test]
fn test_a_star_no_solution() {
    // Arrange: A grid with no possible solution.
    let mut grid = generate_test_grid((5, 5), vec![]);
    for row in 0..5 {
        grid[row][2] = true; // Block the middle column.
    }

    let start = (0, 0);
    let goal = (4, 4);

    // Act: Perform A*.
    let path = a_star(&grid, start, goal);

    // Assert: Verify that A* returns an empty path when no solution exists.
    assert!(path.is_empty(), "A* should return an empty path when no solution exists.");
}

#[test]
fn test_a_star_target_edge_case() {
    // Arrange: A grid where the target is on the edge of the grid.
    let grid = generate_test_grid((5, 5), vec![]);
    let start = (2, 2);
    let goal = (0, 4); // Top-right corner.

    // Act: Perform A*.
    let path = a_star(&grid, start, goal);

    // Assert: Verify that A* finds a valid path to the edge target.
    assert!(!path.is_empty(), "A* should find a valid path to the edge target.");
    assert_eq!(path.first(), Some(&start), "Path should start at the start position.");
    assert_eq!(path.last(), Some(&goal), "Path should end at the edge target.");
}