#[cfg(test)]
#[allow(dead_code)]
pub fn generate_test_grid(size: (usize, usize), obstacles: Vec<(usize, usize)>) -> Vec<Vec<bool>> {
    let mut grid = vec![vec![false; size.1]; size.0];
    for (row, col) in obstacles {
        grid[row][col] = true; // Place obstacles
    }
    grid
}

#[cfg(test)]
#[allow(dead_code)]
pub fn generate_weighted_test_grid(
    size: (usize, usize),
    default_weight: usize,
    obstacles: Vec<(usize, usize)>,
) -> Vec<Vec<usize>> {
    let mut grid = vec![vec![default_weight; size.1]; size.0];
    for (row, col) in obstacles {
        if row < size.0 && col < size.1 {
            grid[row][col] = usize::MAX; // Place obstacles with "infinite" weight
        }
    }
    grid
}