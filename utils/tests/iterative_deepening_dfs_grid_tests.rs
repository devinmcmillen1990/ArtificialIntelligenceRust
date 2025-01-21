mod search;
use search::grid_generator::generate_balanced_grid;
use utils::search::iterative_deepening_dfs_grid::iterative_deepening_dfs_grid;

#[test]
fn test_iterative_deepening_dfs_grid_balanced_with_target_found() {
    let (graph, start) = generate_balanced_grid(3, 3);
    let (found, position) = iterative_deepening_dfs_grid(&graph, start, &'C', 3);
    assert_eq!(found, true);
    assert_eq!(position, Some((0, 2)));
}

#[test]
fn test_iterative_deepening_dfs_grid_balanced_with_target_not_found() {
    let (graph, start) = generate_balanced_grid(3, 3);
    let (found, position) = iterative_deepening_dfs_grid(&graph, start, &'Z', 3);
    assert_eq!(found, false);
    assert_eq!(position, None);
}