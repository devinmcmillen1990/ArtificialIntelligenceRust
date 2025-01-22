use petgraph::graph::{Graph};
use utils::pathfinding::uniform_cost_search_graph::uniform_cost_search_graph;

// TODO: Update these tests to match the A* tests.
#[test]
fn test_uniform_cost_search_graph_path_found() {
    let mut graph = Graph::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");

    graph.add_edge(a, b, 1);
    graph.add_edge(b, c, 2);
    graph.add_edge(a, c, 5);
    graph.add_edge(c, d, 1);

    let result = uniform_cost_search_graph(&graph, a, d);
    assert_eq!(result, Some((vec![a, b, c, d], 4)));
}

#[test]
fn test_uniform_cost_search_graph_no_path() {
    let mut graph = Graph::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");

    graph.add_edge(a, b, 1);

    let result = uniform_cost_search_graph(&graph, a, c);
    assert_eq!(result, None);
}
