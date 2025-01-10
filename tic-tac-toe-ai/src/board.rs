use crate::game_state::{GameState, Player};

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Board {
    pub grid: [[Option<Player>; 3]; 3],
}

impl Board {
    pub fn new() -> Self {
        Board {
            grid: [[None; 3]; 3],
        }
    }

    pub fn make_move(&mut self, row: usize, col: usize, player: Player) -> bool {
        if self.grid[row][col].is_none() {
            self.grid[row][col] = Some(player);
            true
        } else {
            false
        }
    }

    pub fn get_winner(&self) -> GameState {
        for row in 0..3 {
            if let Some(player) = self.grid[row][0] {
                if self.grid[row][1] == Some(player) && self.grid[row][2] == Some(player) {
                    return GameState::Win(player);
                }
            }
        }
        for col in 0..3 {
            if let Some(player) = self.grid[0][col] {
                if self.grid[1][col] == Some(player) && self.grid[2][col] == Some(player) {
                    return GameState::Win(player);
                }
            }
        }
        if let Some(player) = self.grid[0][0] {
            if self.grid[1][1] == Some(player) && self.grid[2][2] == Some(player) {
                return GameState::Win(player);
            }
        }
        if let Some(player) = self.grid[0][2] {
            if self.grid[1][1] == Some(player) && self.grid[2][0] == Some(player) {
                return GameState::Win(player);
            }
        }
        if self.grid.iter().all(|row| row.iter().all(|cell| cell.is_some())) {
            return GameState::Draw;
        }
        GameState::Ongoing
    }

    pub fn available_moves(&self) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        for row in 0..3 {
            for col in 0..3 {
                if self.grid[row][col].is_none() {
                    moves.push((row, col));
                }
            }
        }
        moves
    }

    pub fn hash_state(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
