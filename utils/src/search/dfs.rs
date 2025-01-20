/// Perform a depth-first search to find the target value in the grid.
/// Returns a tuple: (bool, Option<(usize, usize)>), where the boolean indicates
/// whether the value was found, and the `Option` provides the position if found.
pub fn dfs<T: PartialEq>(
    grid: &[Vec<T>],
    start: (usize, usize),
    target: &T,
) -> (bool, Option<(usize, usize)>) {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut stack = vec![start];

    while let Some((row, col)) = stack.pop() {
        // Skip out-of-bounds or already-visited cells.
        if row >= grid.len() || col >= grid[0].len() || visited[row][col] {
            continue;
        }

        // Mark the cell as visited.
        visited[row][col] = true;

        // Check if the current cell contains the target value.
        if &grid[row][col] == target {
            return (true, Some((row, col)));
        }

        // Add neighbors to the stack (DFS traversal).
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
                stack.push((next_row, next_col));
            }
        }
    }

    (false, None) // Return false and no position if the target is not found.
}
