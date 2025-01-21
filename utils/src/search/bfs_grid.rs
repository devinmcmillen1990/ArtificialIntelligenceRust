use std::collections::VecDeque;

pub fn bfs_grid<T: PartialEq>(
    grid: &[Vec<T>],
    start: (usize, usize),
    target: &T,
) -> (bool, Option<(usize, usize)>) {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some((row, col)) = queue.pop_front() {
        if row >= grid.len() || col >= grid[0].len() || visited[row][col] {
            continue;
        }

        visited[row][col] = true;

        if &grid[row][col] == target {
            return (true, Some((row, col)));
        }

        let neighbors = [
            (row.wrapping_sub(1), col),
            (row + 1, col),          
            (row, col.wrapping_sub(1)),
            (row, col + 1),            
        ];

        for &(next_row, next_col) in &neighbors {
            if next_row < grid.len() && next_col < grid[0].len() && !visited[next_row][next_col] {
                queue.push_back((next_row, next_col));
            }
        }
    }

    (false, None)
}
