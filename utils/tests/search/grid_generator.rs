#[cfg(test)]
#[allow(dead_code)]
pub fn generate_balanced_grid(rows: usize, cols: usize) -> (Vec<Vec<char>>, (usize, usize)) {
    let mut grid = Vec::new();
    let mut current_char = b'A';

    for _ in 0..rows {
        let mut row = Vec::new();
        for _ in 0..cols {
            row.push(current_char as char);
            current_char += 1;
            if current_char > b'Z' {
                current_char = b'A'; // Wrap around after 'Z'
            }
        }
        grid.push(row);
    }

    (grid, (0, 0)) // Return the grid and the starting point
}
