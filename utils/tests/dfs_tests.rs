use utils::search::dfs::dfs;

#[test]
fn test_dfs_search_value_with_single_element_grid_is_found() {
    // Arrange: Create a grid with a target value.
    let grid = vec![vec![true]];
    let start = (0, 0);
    let target_value = true;

    // Act: Perform DFS.
    let (found, position) = dfs(&grid, start, &target_value);

    // Assert: Verify the value is found and the position is correct.
    assert!(found, "DFS should find the target value in the grid.");
    assert_eq!(
        position,
        Some((0, 0)),
        "DFS should return the correct position of the target value."
    );
}

#[test]
fn test_dfs_search_value_with_single_element_grid_is_not_found() {
    // Arrange: Create a grid with a target value.
    let grid = vec![vec![true]];
    let start = (0, 0);
    let target_value = false;

    // Act: Perform DFS.
    let (found, position) = dfs(&grid, start, &target_value);

    // Assert: Verify the value is not found and the position is None.
    assert!(!found, "DFS should not find a value that is not in the grid.");
    assert_eq!(position, None, "DFS should return None when value is not found.");
}

#[test]
fn test_dfs_search_value_found() {
    // Arrange: Create a grid with a target value.
    let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let start = (0, 0);
    let target_value = 5;

    // Act: Perform DFS.
    let (found, position) = dfs(&grid, start, &target_value);

    // Assert: Verify the value is found and the position is correct.
    assert!(found, "DFS should find the target value in the grid.");
    assert_eq!(
        position,
        Some((1, 1)),
        "DFS should return the correct position of the target value."
    );
}

#[test]
fn test_dfs_search_value_not_found() {
    // Arrange: Create a grid without the target value.
    let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let start = (0, 0);
    let target_value = 10;

    // Act: Perform DFS.
    let (found, position) = dfs(&grid, start, &target_value);

    // Assert: Verify the value is not found and the position is None.
    assert!(
        !found,
        "DFS should not find a value that does not exist in the grid."
    );
    assert_eq!(position, None, "DFS should return None when value is not found.");
}

#[test]
fn test_dfs_search_large_grid_value_found() {
    // Arrange: Create a larger grid with strings and include the target value.
    let grid = vec![
        vec!["a", "b", "c", "d", "e"],
        vec!["f", "g", "h", "i", "j"],
        vec!["k", "l", "m", "n", "o"],
        vec!["p", "q", "r", "s", "t"],
        vec!["u", "v", "w", "x", "y"],
    ];
    let start = (0, 0);
    let target_value = "m";

    // Act: Perform DFS.
    let (found, position) = dfs(&grid, start, &target_value);

    // Assert: Verify the value is found and the position is correct.
    assert!(found, "DFS should find the target value 'm' in the grid.");
    assert_eq!(
        position,
        Some((2, 2)),
        "DFS should return the correct position of the target value 'm'."
    );
}

#[test]
fn test_dfs_search_large_grid_value_not_found() {
    // Arrange: Create a larger grid with strings, but exclude the target value.
    let grid = vec![
        vec!["a", "b", "c", "d", "e"],
        vec!["f", "g", "h", "i", "j"],
        vec!["k", "l", "m", "n", "o"],
        vec!["p", "q", "r", "s", "t"],
        vec!["u", "v", "w", "x", "y"],
    ];
    let start = (0, 0);
    let target_value = "z";

    // Act: Perform DFS.
    let (found, position) = dfs(&grid, start, &target_value);

    // Assert: Verify the value is not found and the position is None.
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
