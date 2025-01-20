use utils::search::dfs::dfs_grid;

#[test]
fn test_dfs_grid_search_value_with_single_element_grid_is_found() {
    // Arrange
    let grid = vec![vec![true]];
    let start = (0, 0);
    let target_value = true;

    // Act
    let (found, position) = dfs_grid(&grid, start, &target_value);

    // Assert
    assert!(found, "DFS should find the target value in the grid.");
    assert_eq!(
        position,
        Some((0, 0)),
        "DFS should return the correct position of the target value."
    );
}

#[test]
fn test_dfs_grid_search_value_with_single_element_grid_is_not_found() {
    // Arrange
    let grid = vec![vec![true]];
    let start = (0, 0);
    let target_value = false;

    // Act
    let (found, position) = dfs_grid(&grid, start, &target_value);

    // Assert
    assert!(!found, "DFS should not find a value that is not in the grid.");
    assert_eq!(position, None, "DFS should return None when value is not found.");
}

#[test]
fn test_dfs_grid_search_value_found() {
    // Arrange
    let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let start = (0, 0);
    let target_value = 5;

    // Act
    let (found, position) = dfs_grid(&grid, start, &target_value);

    // Assert
    assert!(found, "DFS should find the target value in the grid.");
    assert_eq!(
        position,
        Some((1, 1)),
        "DFS should return the correct position of the target value."
    );
}

#[test]
fn test_dfs_grid_search_value_not_found() {
    // Arrange
    let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let start = (0, 0);
    let target_value = 10;

    // Act
    let (found, position) = dfs_grid(&grid, start, &target_value);

    // Assert
    assert!(
        !found,
        "DFS should not find a value that does not exist in the grid."
    );
    assert_eq!(position, None, "DFS should return None when value is not found.");
}

#[test]
fn test_dfs_grid_search_large_grid_value_found() {
    // Arrange
    let grid = vec![
        vec!["a", "b", "c", "d", "e"],
        vec!["f", "g", "h", "i", "j"],
        vec!["k", "l", "m", "n", "o"],
        vec!["p", "q", "r", "s", "t"],
        vec!["u", "v", "w", "x", "y"],
    ];
    let start = (0, 0);
    let target_value = "m";

    // Act
    let (found, position) = dfs_grid(&grid, start, &target_value);

    // Assert
    assert!(found, "DFS should find the target value 'm' in the grid.");
    assert_eq!(
        position,
        Some((2, 2)),
        "DFS should return the correct position of the target value 'm'."
    );
}

#[test]
fn test_dfs_grid_search_large_grid_value_not_found() {
    // Arrange
    let grid = vec![
        vec!["a", "b", "c", "d", "e"],
        vec!["f", "g", "h", "i", "j"],
        vec!["k", "l", "m", "n", "o"],
        vec!["p", "q", "r", "s", "t"],
        vec!["u", "v", "w", "x", "y"],
    ];
    let start = (0, 0);
    let target_value = "z";

    // Act
    let (found, position) = dfs_grid(&grid, start, &target_value);

    // Assert
    assert!(
        !found,
        "DFS should not find the target value 'z' in the grid."
    );
    assert_eq!(
        position,
        None,
        "DFS should return None when the target value 'z' is not found."
    );
}
