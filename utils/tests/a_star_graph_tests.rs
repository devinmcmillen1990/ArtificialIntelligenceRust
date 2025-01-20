use petgraph::graph::{Graph};
use utils::pathfinding::a_star_graph::astar_graph;

// TODO: Extract the graph generation logic, maybe even make a struct or something for the mock data.
#[test]
fn test_astar_graph_no_obstacles() {
    // Arrange: Create a simple graph without obstacles.
    let mut graph = Graph::<&str, usize>::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");

    // Add edges with weights.
    graph.add_edge(a, b, 1);
    graph.add_edge(b, c, 1);
    graph.add_edge(c, d, 1);
    graph.add_edge(d, e, 1);

    let start = a;
    let goal = e;

    // Act: Perform A*.
    let path = astar_graph(&graph, start, goal, |_current, _goal| 0);

    // Assert: Verify the path is valid.
    assert!(!path.is_empty(), "A* should find a path.");
    assert_eq!(
        path.first(),
        Some(&start),
        "Path should start at the start node."
    );
    assert_eq!(
        path.last(),
        Some(&goal),
        "Path should end at the goal node."
    );
}

#[test]
fn test_astar_graph_with_cycles() {
    // Arrange: Create a graph with cycles.
    let mut graph = Graph::<&str, usize>::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");

    // Add edges with weights and cycles.
    graph.add_edge(a, b, 1);
    graph.add_edge(b, c, 1);
    graph.add_edge(c, b, 1); // Cycle
    graph.add_edge(c, d, 1);
    graph.add_edge(d, e, 1);

    let start = a;
    let goal = e;

    // Act: Perform A*.
    let path = astar_graph(&graph, start, goal, |_current, _goal| 0);

    // Assert: Verify the path avoids cycles and is valid.
    assert!(!path.is_empty(), "A* should find a valid path.");
    assert_eq!(
        path.first(),
        Some(&start),
        "Path should start at the start node."
    );
    assert_eq!(
        path.last(),
        Some(&goal),
        "Path should end at the goal node."
    );
}

#[test]
fn test_astar_graph_no_solution() {
    // Arrange: Create a graph with no solution.
    let mut graph = Graph::<&str, usize>::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");

    // Add edges that don't connect to the goal.
    graph.add_edge(a, b, 1);
    graph.add_edge(b, c, 1);

    let start = a;
    let goal = graph.add_node("D"); // Disconnected node.

    // Act: Perform A*.
    let path = astar_graph(&graph, start, goal, |_current, _goal| 0);

    // Assert: Verify that A* returns an empty path when no solution exists.
    assert!(
        path.is_empty(),
        "A* should return an empty path when no solution exists."
    );
}

#[test]
fn test_astar_graph_target_edge_case() {
    // Arrange: Create a graph where the target is on the edge.
    let mut graph = Graph::<&str, usize>::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");

    // Add edges with weights.
    graph.add_edge(a, b, 1);
    graph.add_edge(b, c, 1);
    graph.add_edge(c, d, 1);

    let start = a;
    let goal = d; // Edge node.

    // Act: Perform A*.
    let path = astar_graph(&graph, start, goal, |_current, _goal| 0);

    // Assert: Verify the path is valid and reaches the edge target.
    assert!(
        !path.is_empty(),
        "A* should find a valid path to the edge target."
    );
    assert_eq!(
        path.first(),
        Some(&start),
        "Path should start at the start node."
    );
    assert_eq!(
        path.last(),
        Some(&goal),
        "Path should end at the edge target."
    );
}
