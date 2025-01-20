use utils::search::dfs::depth_first_search;
use rand::seq::SliceRandom;

pub fn generate_maze(rows: usize, cols: usize) -> Vec<Vec<bool>> {
    let mut grid = vec![vec![false; cols]; rows];

    let mut visit = |row: usize, col: usize, grid: &mut Vec<Vec<bool>>| {
        grid[row][col] = true;
    };

    depth_first_search(&mut grid, 0, 0, &mut visit);

    grid
}
