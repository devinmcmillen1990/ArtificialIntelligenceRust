use crate::board::Board;
use crate::game_state::Player;
use crate::cache::cache_minimax::MinimaxCache;

/// Determines the best move for the given player using the minimax algorithm.
///
/// # Parameters
/// - `board`: A mutable reference to the current game board.
/// - `player`: The player making the move (X or O).
/// - `cache`: A mutable reference to the minimax cache for storing previously evaluated states.
///
/// # Returns
/// A tuple `(usize, usize)` representing the row and column of the best move.
pub fn best_move(board: &mut Board, player: Player, cache: &mut MinimaxCache) -> (usize, usize) {
    let mut best_score = i32::MIN;
    let mut move_to_make = (0, 0);

    for (row, col) in board.available_moves() {
        board.make_move(row, col, &player);
        let score = minimax(board, 0, player == Player::O, cache);
        board.grid[row][col] = None; // Undo the move.

        if score > best_score {
            best_score = score;
            move_to_make = (row, col);
        }
    }

    move_to_make
}

/// Recursive minimax algorithm to evaluate the board state.
///
/// # Parameters
/// - `board`: A mutable reference to the current game board.
/// - `depth`: The depth of the recursion, used to prioritize quicker wins.
/// - `maximizing`: A boolean indicating if the current player is maximizing (true for X, false for O).
/// - `cache`: A mutable reference to the minimax cache for storing previously evaluated states.
///
/// # Returns
/// An `i32` score representing the evaluation of the board state.
fn minimax(board: &mut Board, depth: i32, maximizing: bool, cache: &mut MinimaxCache) -> i32 {
    let hash = board.hash_state();
    if let Some(&cached_score) = cache.map.get(&hash) {
        return cached_score; // Return cached score if available.
    }

    let score = match board.get_winner() {
        crate::game_state::GameState::Win(Player::X) => 10 - depth, // X wins.
        crate::game_state::GameState::Win(Player::O) => depth - 10, // O wins.
        crate::game_state::GameState::Draw => 0,                    // Draw.
        crate::game_state::GameState::Ongoing => {
            let mut best_score = if maximizing { i32::MIN } else { i32::MAX };

            for (row, col) in board.available_moves() {
                board.make_move(row, col, if maximizing { &Player::X } else { &Player::O });
                let child_score = minimax(board, depth + 1, !maximizing, cache);
                board.grid[row][col] = None; // Undo the move.

                if maximizing {
                    best_score = best_score.max(child_score);
                } else {
                    best_score = best_score.min(child_score);
                }
            }

            best_score
        }
    };

    cache.map.insert(hash, score); // Cache the evaluated score.
    score
}
