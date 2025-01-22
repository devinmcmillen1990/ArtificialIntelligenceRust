use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::EdgeRef;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
struct NodeCost<T> {
    node: NodeIndex,
    cost: T,
}

impl<T: PartialEq> Eq for NodeCost<T> {}

impl<T: PartialOrd> PartialOrd for NodeCost<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost) // Reverse to make BinaryHeap a min-heap
    }
}

impl<T: Ord> Ord for NodeCost<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse to make BinaryHeap a min-heap
    }
}

pub fn uniform_cost_search_graph<T>(
    graph: &Graph<T, usize>,
    start: NodeIndex,
    goal: NodeIndex,
) -> Option<(Vec<NodeIndex>, usize)>
where
    T: Clone,
{
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<NodeIndex, NodeIndex> = HashMap::new();
    let mut cost_so_far: HashMap<NodeIndex, usize> = HashMap::new();

    open_set.push(NodeCost { node: start, cost: 0 });
    cost_so_far.insert(start, 0);

    while let Some(NodeCost { node: current, cost: current_cost }) = open_set.pop() {
        if current == goal {
            let mut path = vec![current];
            let mut current_node = current;

            while let Some(&parent) = came_from.get(&current_node) {
                path.push(parent);
                current_node = parent;
            }

            path.reverse();
            return Some((path, current_cost));
        }

        for edge in graph.edges(current) {
            let next = edge.target();
            let edge_cost = *edge.weight();
            let new_cost = current_cost + edge_cost;

            if !cost_so_far.contains_key(&next) || new_cost < cost_so_far[&next] {
                cost_so_far.insert(next, new_cost);
                came_from.insert(next, current);
                open_set.push(NodeCost { node: next, cost: new_cost });
            }
        }
    }

    None // Goal not reachable
}