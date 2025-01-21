use petgraph::graph::{Graph, NodeIndex};

pub fn iterative_deepening_dfs_graph<T>(
    graph: &Graph<T, ()>,
    start: NodeIndex,
    target: Option<&T>,
    max_depth: usize,
) -> Result<Vec<T>, Option<T>>
where
    T: PartialEq + Clone,
{
    let mut visited = Vec::new();

    for depth in 0..=max_depth {
        let mut current_visited = Vec::new();
        if depth_limited_dfs(graph, start, target, depth, &mut current_visited) {
            if target.is_some() {
                return Err(Some(current_visited.last().unwrap().clone()));
            }
        }
        visited.extend(current_visited);
    }

    if target.is_none() {
        Ok(visited) // Return all visited nodes
    } else {
        Err(None) // Target not found
    }
}

/// Helper function to perform Depth-Limited Search (DLS).
fn depth_limited_dfs<T>(
    graph: &Graph<T, ()>,
    node: NodeIndex,
    target: Option<&T>,
    depth: usize,
    visited: &mut Vec<T>,
) -> bool
where
    T: PartialEq + Clone,
{
    if depth == 0 {
        return false; // Depth limit reached
    }

    let value = graph[node].clone();
    visited.push(value.clone());

    if let Some(t) = target {
        if &value == t {
            return true; // Target found
        }
    }

    for neighbor in graph.neighbors(node) {
        if depth_limited_dfs(graph, neighbor, target, depth - 1, visited) {
            return true; // Target found in neighbor
        }
    }

    false // Target not found at this level
}
