use petgraph::graph::{Graph};
use utils::search::dfs::dfs_graph;

#[test]
fn test_dfs_graph_target_found() {
    // Arrange: Create a graph with nodes and edges.
    let mut graph = Graph::<&str, ()>::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    graph.add_edge(a, b, ());
    graph.add_edge(b, c, ());

    let target = "C";

    // Act: Perform DFS to search for a target node.
    let result = dfs_graph(&graph, a, Some(&target));

    // Assert: Verify the target node is found.
    assert_eq!(
        result,
        Err(Some("C")),
        "DFS should find the target node 'C' in the graph."
    );
}

#[test]
fn test_dfs_graph_target_not_found() {
    // Arrange: Create a graph with nodes and edges.
    let mut graph = Graph::<&str, ()>::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    graph.add_edge(a, b, ());
    graph.add_edge(b, c, ());

    let target = "D";

    // Act: Perform DFS to search for a target node that doesn't exist.
    let result = dfs_graph(&graph, a, Some(&target));

    // Assert: Verify the target node is not found.
    assert_eq!(
        result,
        Err(None),
        "DFS should return Err(None) when the target node 'D' is not in the graph."
    );
}

#[test]
fn test_dfs_graph_return_all_nodes() {
    // Arrange: Create a graph with nodes and edges.
    let mut graph = Graph::<&str, ()>::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    graph.add_edge(a, b, ());
    graph.add_edge(b, c, ());

    // Act: Perform DFS without a target to visit all nodes.
    let result = dfs_graph(&graph, a, None);

    // Assert: Verify all nodes are visited in the correct order.
    assert_eq!(
        result,
        Ok(vec!["A", "B", "C"]),
        "DFS should return all visited nodes in the graph."
    );
}

#[test]
fn test_dfs_graph_disconnected_graph_target_found() {
    // Arrange: Create a disconnected graph with nodes and edges.
    let mut graph = Graph::<&str, ()>::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    graph.add_edge(a, b, ());
    graph.add_edge(b, c, ());

    let target = "C";

    // Act: Perform DFS to search for a target node in the same connected component.
    let result = dfs_graph(&graph, a, Some(&target));

    // Assert: Verify the target node is found.
    assert_eq!(
        result,
        Err(Some("C")),
        "DFS should find the target node 'C' in the connected component."
    );

    // Act: Perform DFS to search for a target node in a disconnected component.
    let target = "D";
    let result_disconnected = dfs_graph(&graph, a, Some(&target));

    // Assert: Verify the target node in the disconnected component is not found.
    assert_eq!(
        result_disconnected,
        Err(None),
        "DFS should not find the target node 'D' in a disconnected component."
    );
}

#[test]
fn test_dfs_graph_cyclic_graph() {
    // Arrange: Create a cyclic graph with nodes and edges.
    let mut graph = Graph::<&str, ()>::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    graph.add_edge(a, b, ());
    graph.add_edge(b, c, ());
    graph.add_edge(c, a, ());

    let target = "C";

    // Act: Perform DFS to search for a target node.
    let result = dfs_graph(&graph, a, Some(&target));

    // Assert: Verify the target node is found.
    assert_eq!(
        result,
        Err(Some("C")),
        "DFS should find the target node 'C' in the cyclic graph."
    );
}

#[test]
fn test_dfs_graph_large_graph() {
    // Arrange: Create a larger graph with multiple nodes and edges.
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

    // Act: Perform DFS to search for a target node.
    let result = dfs_graph(&graph, a, Some(&4));

    // Assert: Verify the target node is found.
    assert_eq!(
        result,
        Err(Some(4)),
        "DFS should find the target node 4 in the large graph."
    );

    // Act: Perform DFS without a target to visit all nodes.
    let result_all_nodes = dfs_graph(&graph, a, None);

    // Assert: Verify all nodes are visited in the correct order.
    assert_eq!(
        result_all_nodes,
        Ok(vec![1, 2, 4, 3, 5]),
        "DFS should visit all nodes in the graph."
    );
}
