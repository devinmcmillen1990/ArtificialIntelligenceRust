mod search;
use search::grid_generator::generate_balanced_grid;
use utils::search::dfs_grid::dfs_grid;

#[test]
fn test_dfs_graph_balanced_with_target_found() {
    let (graph, start) = generate_balanced_grid(3, 3);
    let (found, position) = dfs_grid(&graph, start, &'C');
    assert_eq!(found, true);
    assert_eq!(position, Some((0, 2)));
}

#[test]
fn test_dfs_graph_balanced_with_target_not_found() {
    let (graph, start) = generate_balanced_grid(3, 3);
    let (found, position) = dfs_grid(&graph, start, &'Z');
    assert_eq!(found, false);
    assert_eq!(position, None);
}