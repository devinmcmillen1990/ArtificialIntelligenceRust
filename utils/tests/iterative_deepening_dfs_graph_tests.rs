mod search;
use search::graph_generator::{
    generate_balanced_graph, generate_balanced_graph_with_cycles, generate_unbalanced_graph,
};
use utils::search::iterative_deepening_dfs_graph::iterative_deepening_dfs_graph;

#[test]
fn test_iterative_deepening_dfs_graph_target_found() {
    let (graph, start) = generate_balanced_graph(3);
    let result = iterative_deepening_dfs_graph(&graph, start, Some(&'C'), 3);
    assert_eq!(result, Err(Some('C')));
}

#[test]
fn test_iterative_deepening_dfs_graph_target_not_found() {
    let (graph, start) = generate_balanced_graph(3);
    let result = iterative_deepening_dfs_graph(&graph, start, Some(&'D'), 3);
    assert_eq!(result, Err(None));
}

#[test]
fn test_iterative_deepening_dfs_graph_return_all_nodes() {
    let (graph, start) = generate_balanced_graph(3);
    let result = iterative_deepening_dfs_graph(&graph, start, None, 3);
    assert_eq!(result, Ok(vec!['A', 'A', 'C', 'B', 'A', 'C', 'B']));
}

#[test]
fn test_iterative_deepening_dfs_graph_unbalanced_with_target_found() {
    let (graph, start) = generate_unbalanced_graph(3);
    let result = iterative_deepening_dfs_graph(&graph, start, Some(&'C'), 3);
    assert_eq!(result, Err(Some('C')));
}

#[test]
fn test_iterative_deepening_dfs_graph_unbalanced_with_target_not_found() {
    let (graph, start) = generate_unbalanced_graph(3);
    let result = iterative_deepening_dfs_graph(&graph, start, Some(&'D'), 3);
    assert_eq!(result, Err(None));
}

#[test]
fn test_iterative_deepening_dfs_graph_unbalanced_return_all_nodes() {
    let (graph, start) = generate_unbalanced_graph(3);
    let result = iterative_deepening_dfs_graph(&graph, start, None, 3);
    assert_eq!(result, Ok(vec!['A', 'A', 'B', 'A', 'B', 'C']));
}

#[test]
fn test_iterative_deepening_dfs_graph_balanced_with_cycles_with_target_found() {
    let (graph, start) = generate_balanced_graph_with_cycles(3);
    let result = iterative_deepening_dfs_graph(&graph, start, Some(&'C'), 3);
    assert_eq!(result, Err(Some('C')));
}

#[test]
fn test_iterative_deepening_dfs_graph_balanced_with_cycles_with_target_not_found() {
    let (graph, start) = generate_balanced_graph_with_cycles(3);
    let result = iterative_deepening_dfs_graph(&graph, start, Some(&'D'), 3);
    assert_eq!(result, Err(None));
}

#[test]
fn test_iterative_deepening_dfs_graph_balanced_with_cycles_return_all_nodes() {
    let (graph, start) = generate_balanced_graph_with_cycles(3);
    let result = iterative_deepening_dfs_graph(&graph, start, None, 3);
    assert_eq!(result, Ok(vec!['A', 'A', 'C', 'B', 'A', 'C', 'A', 'B', 'A']));
}
