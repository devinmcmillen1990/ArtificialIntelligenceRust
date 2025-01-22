use std::collections::{BinaryHeap};
use std::cmp::Reverse;

pub fn uniform_cost_search_grid(
    grid: &[Vec<usize>],
    start: (usize, usize),
    goal: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut cost_so_far = vec![vec![usize::MAX; grid[0].len()]; grid.len()];
    let mut came_from = vec![vec![None; grid[0].len()]; grid.len()];
    let mut frontier: BinaryHeap<Reverse<(usize, (usize, usize))>> = BinaryHeap::new(); // Specify type for frontier

    frontier.push(Reverse((0, start)));
    cost_so_far[start.0][start.1] = 0;

    while let Some(Reverse((current_cost, (row, col)))) = frontier.pop() {
        if (row, col) == goal {
            break;
        }

        let neighbors = [
            (row.wrapping_sub(1), col), // Up
            (row + 1, col),             // Down
            (row, col.wrapping_sub(1)), // Left
            (row, col + 1),             // Right
        ];

        for &(next_row, next_col) in &neighbors {
            if next_row < grid.len() && next_col < grid[0].len() {
                // Skip impassable cells
                if grid[next_row][next_col] == usize::MAX {
                    continue;
                }

                let new_cost = current_cost.saturating_add(grid[next_row][next_col]);
                if new_cost < cost_so_far[next_row][next_col] {
                    cost_so_far[next_row][next_col] = new_cost;
                    frontier.push(Reverse((new_cost, (next_row, next_col))));
                    came_from[next_row][next_col] = Some((row, col));
                }
            }
        }
    }

    // Reconstruct the path
    let mut path = Vec::new();
    let mut current = Some(goal);

    while let Some(pos) = current {
        path.push(pos);
        current = came_from[pos.0][pos.1];
    }

    path.reverse();

    if path.first() == Some(&start) && path.last() == Some(&goal) {
        path
    } else {
        Vec::new()
    }
}
