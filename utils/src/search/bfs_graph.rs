use petgraph::graph::{Graph, NodeIndex};
use std::collections::VecDeque;

pub fn bfs_graph<T>(
    graph: &Graph<T, ()>,
    start: NodeIndex,
    target: Option<&T>,
) -> Result<Vec<T>, Option<T>>
where
    T: PartialEq + Clone,
{
    let mut visited = Vec::new();
    let mut queue = VecDeque::new();
    let mut discovered = vec![false; graph.node_count()];

    queue.push_back(start);
    discovered[start.index()] = true;

    while let Some(node) = queue.pop_front() {
        let value = graph[node].clone();
        visited.push(value.clone());

        if let Some(t) = target {
            if &value == t {
                return Err(Some(value));
            }
        }

        for neighbor in graph.neighbors(node) {
            if !discovered[neighbor.index()] {
                queue.push_back(neighbor);
                discovered[neighbor.index()] = true;
            }
        }
    }

    if target.is_none() {
        Ok(visited)
    } else {
        Err(None)
    }
}