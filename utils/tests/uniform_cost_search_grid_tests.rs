mod pathfinding;
use pathfinding::grid_generator::generate_weighted_test_grid;
use utils::pathfinding::uniform_cost_search_grid::uniform_cost_search_grid;

#[test]
fn test_uniform_cost_search_grid_no_obstacles() {
    // Arrange
    let grid = generate_weighted_test_grid((5, 5), 1, vec![]);
    let start = (0, 0);
    let goal = (4, 4);

    // Act
    let path = uniform_cost_search_grid(&grid, start, goal);

    // Assert
    assert!(!path.is_empty());
    assert_eq!(path.first(), Some(&start));
    assert_eq!(path.last(), Some(&goal));
}

#[test]
fn test_uniform_cost_search_grid_with_cycles() {
    // Arrange
    let grid = generate_weighted_test_grid(
        (5, 5),
        1,
        vec![(1, 1), (1, 2), (1, 3), (2, 1), (3, 1), (3, 2), (3, 3)],
    );

    let start = (0, 0);
    let goal = (4, 4);

    // Act
    let path = uniform_cost_search_grid(&grid, start, goal);

    // Assert
    assert!(!path.is_empty());
    assert_eq!(path.first(), Some(&start));
    assert_eq!(path.last(), Some(&goal));
}

#[test]
fn test_uniform_cost_search_grid_no_solution() {
    // Arrange
    let grid =
        generate_weighted_test_grid((5, 5), 1, vec![(0, 2), (1, 2), (2, 2), (3, 2), (4, 2)]);

    let start = (0, 0);
    let goal = (4, 4);

    // Act
    let path = uniform_cost_search_grid(&grid, start, goal);

    // Assert
    assert!(path.is_empty());
}

#[test]
fn test_uniform_cost_search_grid_target_edge_case() {
    // Arrange
    let grid = generate_weighted_test_grid((5, 5), 1, vec![]);
    let start = (2, 2);
    let goal = (0, 4); // Top-right corner.

    // Act
    let path = uniform_cost_search_grid(&grid, start, goal);

    // Assert
    assert!(!path.is_empty());
    assert_eq!(path.first(), Some(&start));
    assert_eq!(path.last(), Some(&goal));
}
