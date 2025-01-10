use crate::game_state::{GameState, Player};

/// Represents the tic-tac-toe board and provides methods to manage game state.
#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Board {
    /// The 3x3 grid representing the board.
    /// Each cell is an `Option<Player>`, where `None` means the cell is empty.
    pub grid: [[Option<Player>; 3]; 3],
    /// Cached hash of the board state to avoid recomputation.
    cached_hash: Option<u64>,
}

impl Board {
    /// Creates a new empty board.
    /// All cells are initialized to `None`.
    pub fn new() -> Self {
        Board {
            grid: [[None; 3]; 3],
            cached_hash: None,
        }
    }

    /// Attempts to make a move on the board.
    ///
    /// # Parameters
    /// - `row`: The row index of the move (0-based).
    /// - `col`: The column index of the move (0-based).
    /// - `player`: A reference to the `Player` making the move (X or O).
    ///
    /// # Returns
    /// - `true` if the move was successful (cell was empty).
    /// - `false` if the cell was already occupied.
    pub fn make_move(&mut self, row: usize, col: usize, player: &Player) -> bool {
        if self.grid[row][col].is_none() {
            self.grid[row][col] = Some(*player);
            self.cached_hash = None; // Invalidate the cached hash.
            true
        } else {
            false
        }
    }

    /// Determines the current game state.
    ///
    /// # Returns
    /// - `GameState::Win(Player)` if a player has won.
    /// - `GameState::Draw` if the board is full with no winner.
    /// - `GameState::Ongoing` if the game is still in progress.
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
        if self
            .grid
            .iter()
            .all(|row| row.iter().all(|cell| cell.is_some()))
        {
            return GameState::Draw;
        }
        GameState::Ongoing
    }

    /// Retrieves all available moves on the board.
    ///
    /// # Returns
    /// An iterator over tuples `(row, col)` representing empty cells.
    pub fn available_moves(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.grid.iter().enumerate().flat_map(|(row, cols)| {
            cols.iter().enumerate().filter_map(move |(col, cell)| {
                if cell.is_none() {
                    Some((row, col))
                } else {
                    None
                }
            })
        })
    }

    /// Generates a unique hash for the current board state.
    /// If the hash is already cached, returns the cached value.
    ///
    /// # Returns
    /// A `u64` hash value representing the board state.
    pub fn hash_state(&mut self) -> u64 {
        if let Some(cached) = self.cached_hash {
            return cached;
        }
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        let hash = hasher.finish();
        self.cached_hash = Some(hash);
        hash
    }
}
