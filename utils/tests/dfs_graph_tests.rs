mod search;
use search::graph_generator::{
    generate_balanced_graph, generate_balanced_graph_with_cycles, generate_unbalanced_graph,
};
use utils::search::dfs_graph::dfs_graph;

#[test]
fn test_dfs_graph_target_found() {
    let (graph, start) = generate_balanced_graph(3);
    let result = dfs_graph(&graph, start, Some(&'C'));
    assert_eq!(result, Err(Some('C')));
}

#[test]
fn test_dfs_graph_target_not_found() {
    let (graph, start) = generate_balanced_graph(3);
    let result = dfs_graph(&graph, start, Some(&'D'));
    assert_eq!(result, Err(None));
}

#[test]
fn test_dfs_graph_return_all_nodes() {
    let (graph, start) = generate_balanced_graph(3);
    let result = dfs_graph(&graph, start, None);
    assert_eq!(result, Ok(vec!['A', 'B', 'C']));
}

#[test]
fn test_dfs_graph_unbalanced_with_target_found() {
    let (graph, start) = generate_unbalanced_graph(3);
    let result = dfs_graph(&graph, start, Some(&'C'));
    assert_eq!(result, Err(Some('C')));
}

#[test]
fn test_dfs_graph_unbalanced_with_target_not_found() {
    let (graph, start) = generate_unbalanced_graph(3);
    let result = dfs_graph(&graph, start, Some(&'D'));
    assert_eq!(result, Err(None));
}

#[test]
fn test_dfs_graph_unbalanced_return_all_nodes() {
    let (graph, start) = generate_unbalanced_graph(3);
    let result = dfs_graph(&graph, start, None);
    assert_eq!(result, Ok(vec!['A', 'B', 'C']));
}

#[test]
fn test_dfs_graph_balanced_with_cycles_with_target_found() {
    let (graph, start) = generate_balanced_graph_with_cycles(3);
    let result = dfs_graph(&graph, start, Some(&'C'));
    assert_eq!(result, Err(Some('C')));
}

#[test]
fn test_dfs_graph_balanced_with_cycles_with_target_not_found() {
    let (graph, start) = generate_balanced_graph_with_cycles(3);
    let result = dfs_graph(&graph, start, Some(&'D'));
    assert_eq!(result, Err(None));
}

#[test]
fn test_dfs_graph_balanced_with_cycles_return_all_nodes() {
    let (graph, start) = generate_balanced_graph_with_cycles(3);
    let result = dfs_graph(&graph, start, None);
    assert_eq!(result, Ok(vec!['A', 'B', 'C']));
}
