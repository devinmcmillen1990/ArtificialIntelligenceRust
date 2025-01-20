use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::{EdgeRef};
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
struct NodeCost<T>
where
    T: PartialEq + Eq + Clone + std::hash::Hash,
{
    cost: usize,
    index: NodeIndex,
    estimated_cost: usize,
    marker: std::marker::PhantomData<T>,
}

impl<T> Ord for NodeCost<T>
where
    T: PartialEq + Eq + Clone + std::hash::Hash,
{
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .estimated_cost
            .cmp(&self.estimated_cost)
            .then_with(|| self.cost.cmp(&other.cost))
    }
}

impl<T> PartialOrd for NodeCost<T>
where
    T: PartialEq + Eq + Clone + std::hash::Hash,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A* algorithm for a graph.
///
/// # Parameters
/// - `graph`: The graph to perform A* on.
/// - `start`: The starting node index.
/// - `goal`: The goal node index.
/// - `heuristic`: A closure that calculates the heuristic cost between two nodes.
///
/// # Returns
/// - A vector of node indices representing the shortest path, or an empty vector if no path exists.
pub fn astar_graph<T, F>(
    graph: &Graph<T, usize>,
    start: NodeIndex,
    goal: NodeIndex,
    heuristic: F,
) -> Vec<NodeIndex>
where
    T: PartialEq + Eq + Clone + std::hash::Hash,
    F: Fn(NodeIndex, NodeIndex) -> usize,
{
    let mut open_set: BinaryHeap<NodeCost<T>> = BinaryHeap::new();
    let mut g_scores = HashMap::new();
    let mut came_from = HashMap::new();

    g_scores.insert(start, 0);
    open_set.push(NodeCost {
        cost: 0,
        index: start,
        estimated_cost: heuristic(start, goal),
        marker: std::marker::PhantomData,
    });

    while let Some(NodeCost { index, cost, .. }) = open_set.pop() {
        if index == goal {
            let mut path = Vec::new();
            let mut current = Some(index);
            while let Some(node) = current {
                path.push(node);
                current = came_from.get(&node).copied();
            }
            path.reverse();
            return path;
        }

        if cost > *g_scores.get(&index).unwrap_or(&usize::MAX) {
            continue;
        }

        for edge in graph.edges(index) {
            let neighbor = edge.target();
            let tentative_g_score = cost + edge.weight();
            if tentative_g_score < *g_scores.get(&neighbor).unwrap_or(&usize::MAX) {
                came_from.insert(neighbor, index);
                g_scores.insert(neighbor, tentative_g_score);
                open_set.push(NodeCost {
                    cost: tentative_g_score,
                    index: neighbor,
                    estimated_cost: tentative_g_score + heuristic(neighbor, goal),
                    marker: std::marker::PhantomData,
                });
            }
        }
    }

    Vec::new() // Return an empty path if no solution exists.
}
