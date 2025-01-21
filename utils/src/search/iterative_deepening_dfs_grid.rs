pub fn iterative_deepening_dfs_grid<T: PartialEq>(
    grid: &[Vec<T>],
    start: (usize, usize),
    target: &T,
    max_depth: usize,
) -> (bool, Option<(usize, usize)>) {
    for depth in 0..=max_depth {
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let mut visited_positions = Vec::new();
        if let Some(position) =
            depth_limited_dfs_grid(grid, start, target, depth, &mut visited, &mut visited_positions)
        {
            return (true, Some(position)); // Return the correct position of the target
        }
    }

    (false, None) // Target not found
}

fn depth_limited_dfs_grid<T: PartialEq>(
    grid: &[Vec<T>],
    position: (usize, usize),
    target: &T,
    depth: usize,
    visited: &mut Vec<Vec<bool>>,
    visited_positions: &mut Vec<(usize, usize)>,
) -> Option<(usize, usize)> {
    let (row, col) = position;

    if depth == 0 || row >= grid.len() || col >= grid[0].len() || visited[row][col] {
        return None;
    }

    visited[row][col] = true;
    visited_positions.push((row, col));

    if &grid[row][col] == target {
        return Some(position); // Return the target position
    }

    let neighbors = [
        (row.wrapping_sub(1), col), // Up
        (row + 1, col),             // Down
        (row, col.wrapping_sub(1)), // Left
        (row, col + 1),             // Right
    ];

    for &(next_row, next_col) in &neighbors {
        if next_row < grid.len()
            && next_col < grid[0].len()
            && !visited[next_row][next_col]
        {
            if let Some(found_position) = depth_limited_dfs_grid(
                grid,
                (next_row, next_col),
                target,
                depth - 1,
                visited,
                visited_positions,
            ) {
                return Some(found_position); // Propagate the target position upwards
            }
        }
    }

    None // Target not found
}
