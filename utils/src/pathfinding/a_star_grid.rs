use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    position: (usize, usize),
    cost: usize, // The cost to reach this node (g(n))
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering to make BinaryHeap a min-heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Heuristic function: Manhattan distance
fn heuristic(a: (usize, usize), b: (usize, usize)) -> usize {
    let (x1, y1) = a;
    let (x2, y2) = b;
    (x1 as isize - x2 as isize).abs() as usize + (y1 as isize - y2 as isize).abs() as usize
}

pub fn a_star_grid(
    grid: &[Vec<bool>],
    start: (usize, usize),
    goal: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut g_score: HashMap<(usize, usize), usize> = HashMap::new();

    open_set.push(Node {
        position: start,
        cost: heuristic(start, goal),
    });
    g_score.insert(start, 0);

    while let Some(Node { position, .. }) = open_set.pop() {
        if position == goal {
            // Reconstruct the path
            let mut current = position;
            let mut path = vec![current];
            while let Some(&prev) = came_from.get(&current) {
                current = prev;
                path.push(current);
            }
            path.reverse();
            return path;
        }

        let (row, col) = position;

        // Neighbors: Up, Down, Left, Right
        let neighbors = [
            (row.wrapping_sub(1), col),
            (row + 1, col),
            (row, col.wrapping_sub(1)),
            (row, col + 1),
        ];

        for &(next_row, next_col) in &neighbors {
            if next_row < grid.len() && next_col < grid[0].len() && !grid[next_row][next_col] {
                let tentative_g_score = g_score.get(&position).unwrap_or(&usize::MAX) + 1;

                if tentative_g_score < *g_score.get(&(next_row, next_col)).unwrap_or(&usize::MAX) {
                    g_score.insert((next_row, next_col), tentative_g_score);
                    came_from.insert((next_row, next_col), position);
                    let f_score = tentative_g_score + heuristic((next_row, next_col), goal);
                    open_set.push(Node {
                        position: (next_row, next_col),
                        cost: f_score,
                    });
                }
            }
        }
    }

    // If we exhaust the open set without finding the goal, return an empty path.
    Vec::new()
}
