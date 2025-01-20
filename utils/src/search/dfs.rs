pub fn dfs<T: PartialEq>(
    grid: &[Vec<T>],
    start: (usize, usize),
    target_value: &T,
) -> bool {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    dfs_helper(grid, start, target_value, &mut visited)
}

fn dfs_helper<T: PartialEq>(
    grid: &[Vec<T>],
    current: (usize, usize),
    target_value: &T,
    visited: &mut Vec<Vec<bool>>,
) -> bool {
    let (row, col) = current;

    // Check bounds and whether the cell is already visited.
    if row >= grid.len() || col >= grid[0].len() || visited[row][col] {
        return false;
    }

    // Mark the cell as visited.
    visited[row][col] = true;

    // Check if the current cell contains the target value.
    if &grid[row][col] == target_value {
        return true;
    }

    // Explore neighbors (up, down, left, right).
    let neighbors = vec![
        (row.wrapping_sub(1), col), // Up
        (row + 1, col),             // Down
        (row, col.wrapping_sub(1)), // Left
        (row, col + 1),             // Right
    ];

    for (next_row, next_col) in neighbors {
        if dfs_helper(grid, (next_row, next_col), target_value, visited) {
            return true;
        }
    }

    false
}
