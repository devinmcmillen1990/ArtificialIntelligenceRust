use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::Dfs;

/// Perform a depth-first search on a graph to find a target node.
///
/// # Parameters
/// - `graph`: The graph to traverse.
/// - `start`: The starting node index for the DFS.
/// - `target`: Optional target value to search for.
///
/// # Returns
/// - `Err(Some(T))`: If the target is found, contains the value of the node.
/// - `Err(None)`: If the target is not found and a search was attempted.
/// - `Ok(Vec<T>)`: If no target is specified, returns all visited node values.
pub fn dfs_graph<T>(
    graph: &Graph<T, ()>,
    start: NodeIndex,
    target: Option<&T>,
) -> Result<Vec<T>, Option<T>>
where
    T: PartialEq + Clone,
{
    let mut dfs = Dfs::new(graph, start);
    let mut visited = Vec::new();

    while let Some(node) = dfs.next(graph) {
        let value = graph[node].clone();
        visited.push(value.clone());

        if let Some(t) = target {
            if &value == t {
                return Err(Some(value)); // Found the target node, return its value.
            }
        }
    }

    if target.is_none() {
        Ok(visited) // Return all visited nodes when no target is specified.
    } else {
        Err(None) // Target not found.
    }
}

/// Perform a depth-first search to find the target value in the grid.
/// Returns a tuple: (bool, Option<(usize, usize)>), where the boolean indicates
/// whether the value was found, and the `Option` provides the position if found.
pub fn dfs_grid<T: PartialEq>(
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
            if next_row < grid.len() && next_col < grid[0].len() && !visited[next_row][next_col] {
                stack.push((next_row, next_col));
            }
        }
    }

    (false, None) // Return false and no position if the target is not found.
}
