use petgraph::graph::{Graph, NodeIndex};

pub fn generate_balanced_graph(size: usize) -> (Graph<char, ()>, NodeIndex) {
    let mut graph = Graph::<char, ()>::new();
    let mut nodes = Vec::new();

    // Create nodes
    for i in 0..size {
        let node = graph.add_node((b'A' + i as u8) as char);
        nodes.push(node);
    }

    // Create balanced edges
    for i in 0..(size / 2) {
        let left_child = 2 * i + 1;
        let right_child = 2 * i + 2;

        if left_child < size {
            graph.add_edge(nodes[i], nodes[left_child], ());
        }
        if right_child < size {
            graph.add_edge(nodes[i], nodes[right_child], ());
        }
    }

    (graph, nodes[0]) // Return the graph and the first node
}

pub fn generate_balanced_graph_with_cycles(size: usize) -> (Graph<char, ()>, NodeIndex) {
    let mut graph = Graph::<char, ()>::new();
    let mut nodes = Vec::new();

    // Create nodes
    for i in 0..size {
        let node = graph.add_node((b'A' + i as u8) as char);
        nodes.push(node);
    }

    // Create balanced edges
    for i in 0..(size / 2) {
        let left_child = 2 * i + 1;
        let right_child = 2 * i + 2;

        if left_child < size {
            graph.add_edge(nodes[i], nodes[left_child], ());
            graph.add_edge(nodes[left_child], nodes[i], ());
        }
        if right_child < size {
            graph.add_edge(nodes[i], nodes[right_child], ());
            graph.add_edge(nodes[right_child], nodes[i], ());
        }
    }

    (graph, nodes[0]) // Return the graph and the first node
}

pub fn generate_unbalanced_graph(size: usize) -> (Graph<char, ()>, NodeIndex) {
    let mut graph = Graph::<char, ()>::new();
    let mut nodes = Vec::new();

    // Create nodes
    for i in 0..size {
        let node = graph.add_node((b'A' + i as u8) as char);
        nodes.push(node);
    }

    // Create unbalanced edges
    for i in 0..(size - 1) {
        graph.add_edge(nodes[i], nodes[i + 1], ());
    }

    (graph, nodes[0]) // Return the graph and the first node
}
