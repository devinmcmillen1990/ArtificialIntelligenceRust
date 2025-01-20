pub fn generate_test_grid(size: (usize, usize), obstacles: Vec<(usize, usize)>) -> Vec<Vec<bool>> {
    let mut grid = vec![vec![false; size.1]; size.0];
    for (row, col) in obstacles {
        grid[row][col] = true; // Place obstacles
    }
    grid
}
