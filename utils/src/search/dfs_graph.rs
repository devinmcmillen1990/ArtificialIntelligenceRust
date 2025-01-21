use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::Dfs;

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
                return Err(Some(value));
            }
        }
    }

    if target.is_none() {
        Ok(visited)
    } else {
        Err(None)
    }
}