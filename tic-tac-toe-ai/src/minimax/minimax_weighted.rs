use crate::board::Board;
use crate::cache::cache_minimax_weighted::{WeightedCache, WeightedStateEvaluation};
use crate::game_state::Player;

/// Board weights used to guide the weighted minimax algorithm.
const WEIGHTS: [[i32; 3]; 3] = [
    [3, 2, 3],
    [2, 5, 2],
    [3, 2, 3],
];

/// Determines the best move for the given player using the weighted minimax algorithm.
///
/// # Parameters
/// - `board`: A mutable reference to the current game board.
/// - `player`: The player making the move (X or O).
/// - `cache`: A mutable reference to the weighted minimax cache for storing previously evaluated states.
///
/// # Returns
/// A tuple `(usize, usize)` representing the row and column of the best move.
pub fn best_weighted_move(
    board: &mut Board,
    player: &Player,
    cache: &mut WeightedCache,
) -> (usize, usize) {
    let mut best_score = i32::MIN;
    let mut move_to_make = (0, 0);

    for (row, col) in board.available_moves() {
        board.make_move(row, col, player);
        let score = weighted_minimax(board, 0, player == &Player::O, cache);
        board.grid[row][col] = None; // Undo the move.

        let weighted_score = score + (WEIGHTS[row][col] * 100);

        println!(
            "Move ({}, {}): Score = {}, Weight = {}, Weighted Score = {}",
            row, col, score, WEIGHTS[row][col], weighted_score
        );

        if weighted_score > best_score {
            best_score = weighted_score;
            move_to_make = (row, col);
        }
    }

    println!(
        "Chosen move: ({}, {}) with weighted score: {}",
        move_to_make.0, move_to_make.1, best_score
    );

    move_to_make
}

/// Recursive weighted minimax algorithm to evaluate the board state.
///
/// # Parameters
/// - `board`: A mutable reference to the current game board.
/// - `depth`: The depth of the recursion, used to prioritize quicker wins.
/// - `maximizing`: A boolean indicating if the current player is maximizing (true for X, false for O).
/// - `cache`: A mutable reference to the weighted minimax cache for storing previously evaluated states.
///
/// # Returns
/// An `i32` score representing the evaluation of the board state.
fn weighted_minimax(
    board: &mut Board,
    depth: i32,
    maximizing: bool,
    cache: &mut WeightedCache,
) -> i32 {
    let hash = board.hash_state();
    if let Some(cached_eval) = cache.map.get(&hash) {
        return cached_eval.score;
    }

    let score = match board.get_winner() {
        crate::game_state::GameState::Win(Player::X) => 10_000 - depth,
        crate::game_state::GameState::Win(Player::O) => depth - 10_000,
        crate::game_state::GameState::Draw => 0,
        crate::game_state::GameState::Ongoing => {
            let mut best_score = if maximizing { i32::MIN } else { i32::MAX };

            for (row, col) in board.available_moves() {
                board.make_move(row, col, if maximizing { &Player::X } else { &Player::O });
                let child_score = weighted_minimax(board, depth + 1, !maximizing, cache);
                board.grid[row][col] = None; // Undo the move.

                let position_weight = WEIGHTS[row][col] * 100;
                let weighted_score = child_score + position_weight;

                if maximizing {
                    best_score = best_score.max(weighted_score);
                } else {
                    best_score = best_score.min(weighted_score);
                }
            }

            best_score
        }
    };

    cache.map.insert(
        hash,
        WeightedStateEvaluation {
            score,
            weights: evaluate_weights(board),
        },
    );

    score
}

/// Evaluates the weights for the current board state.
///
/// # Parameters
/// - `board`: A reference to the current game board.
///
/// # Returns
/// A vector of weights representing the importance of each position for the given board state.
pub fn evaluate_weights(board: &Board) -> Vec<i32> {
    let mut flat_weights = vec![0; 9];
    for (i, row) in WEIGHTS.iter().enumerate() {
        for (j, &weight) in row.iter().enumerate() {
            if board.grid[i][j].is_none() {
                flat_weights[i * 3 + j] = weight;
            }
        }
    }
    flat_weights
}
