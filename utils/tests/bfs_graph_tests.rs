use petgraph::graph::{Graph};
use utils::search::bfs::bfs_graph;

#[test]
fn test_bfs_graph_target_found() {
    // Arrange
    let mut graph = Graph::<&str, ()>::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    graph.add_edge(a, b, ());
    graph.add_edge(b, c, ());

    let target = "C";

    // Act
    let result = bfs_graph(&graph, a, Some(&target));

    // Assert
    assert_eq!(
        result,
        Err(Some("C")),
        "BFS should find the target node 'C' in the graph."
    );
}

#[test]
fn test_bfs_graph_target_not_found() {
    // Arrange
    let mut graph = Graph::<&str, ()>::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    graph.add_edge(a, b, ());
    graph.add_edge(b, c, ());

    let target = "D";

    // Act
    let result = bfs_graph(&graph, a, Some(&target));

    // Assert
    assert_eq!(
        result,
        Err(None),
        "BFS should return Err(None) when the target node 'D' is not in the graph."
    );
}

#[test]
fn test_bfs_graph_return_all_nodes() {
    // Arrange
    let mut graph = Graph::<&str, ()>::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    graph.add_edge(a, b, ());
    graph.add_edge(b, c, ());

    // Act
    let result = bfs_graph(&graph, a, None);

    // Assert
    assert_eq!(
        result,
        Ok(vec!["A", "B", "C"]),
        "BFS should return all visited nodes in the graph."
    );
}

#[test]
fn test_bfs_graph_disconnected_graph_target_found() {
    // Arrange
    let mut graph = Graph::<&str, ()>::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    graph.add_edge(a, b, ());
    graph.add_edge(b, c, ());

    let target = "C";

    // Act
    let result = bfs_graph(&graph, a, Some(&target));

    // Assert
    assert_eq!(
        result,
        Err(Some("C")),
        "BFS should find the target node 'C' in the connected component."
    );

    // Act
    let target = "D";
    let result_disconnected = bfs_graph(&graph, a, Some(&target));

    // Assert
    assert_eq!(
        result_disconnected,
        Err(None),
        "BFS should not find the target node 'D' in a disconnected component."
    );
}

#[test]
fn test_bfs_graph_cyclic_graph() {
    // Arrange
    let mut graph = Graph::<&str, ()>::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    graph.add_edge(a, b, ());
    graph.add_edge(b, c, ());
    graph.add_edge(c, a, ());

    let target = "C";

    // Act
    let result = bfs_graph(&graph, a, Some(&target));

    // Assert
    assert_eq!(
        result,
        Err(Some("C")),
        "BFS should find the target node 'C' in the cyclic graph."
    );
}

#[test]
fn test_bfs_graph_large_graph() {
    // Arrange
    let mut graph = Graph::<i32, ()>::new();
    let a = graph.add_node(1);
    let b = graph.add_node(2);
    let c = graph.add_node(3);
    let d = graph.add_node(4);
    let e = graph.add_node(5);

    graph.add_edge(a, b, ());
    graph.add_edge(a, c, ());
    graph.add_edge(b, d, ());
    graph.add_edge(c, e, ());

    // Act
    let result = bfs_graph(&graph, a, Some(&4));

    // Assert
    assert_eq!(
        result,
        Err(Some(4)),
        "BFS should find the target node 4 in the large graph."
    );

    // Act
    let result_all_nodes = bfs_graph(&graph, a, None);

    // Assert
    let mut visited_nodes = match result_all_nodes {
        Ok(nodes) => nodes,
        Err(_) => panic!("Expected Ok for visiting all nodes, but got Err."),
    };

    visited_nodes.sort(); // Sort nodes to ensure a consistent comparison

    let mut expected_nodes = vec![1, 2, 3, 4, 5];
    expected_nodes.sort();

    assert_eq!(
        visited_nodes, expected_nodes,
        "BFS should visit all nodes in the graph, regardless of order."
    );
}
