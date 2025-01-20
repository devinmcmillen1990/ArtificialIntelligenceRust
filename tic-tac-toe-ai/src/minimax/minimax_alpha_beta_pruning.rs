use crate::board::Board;
use crate::game_state::Player;
use crate::cache::cache_minimax_alpha_beta_pruning::AlphaBetaCache;

/// Implements the Minimax algorithm with Alpha-Beta Pruning.
/// Alpha-Beta Pruning optimizes the Minimax algorithm by skipping unnecessary branches
/// that cannot influence the final decision, reducing the computational overhead.
/// 
/// # Arguments
/// * `board` - A mutable reference to the game board.
/// * `depth` - The current depth of the recursion.
/// * `alpha` - The best value that the maximizer currently can guarantee.
/// * `beta` - The best value that the minimizer currently can guarantee.
/// * `maximizing` - A boolean indicating if the current layer is maximizing or minimizing.
/// * `cache` - A mutable reference to an `AlphaBetaCache` for storing previously evaluated states.
/// 
/// # Returns
/// The score of the best possible move.
pub fn alpha_beta_pruning(
    board: &mut Board,
    depth: i32,
    alpha: &mut i32,
    beta: &mut i32,
    maximizing: bool,
    cache: &mut AlphaBetaCache,
) -> i32 {
    let hash = board.hash_state();
    if let Some(&cached_score) = cache.map.get(&hash) {
        return cached_score;
    }

    let score = match board.get_winner() {
        crate::game_state::GameState::Win(Player::X) => 10 - depth,
        crate::game_state::GameState::Win(Player::O) => depth - 10,
        crate::game_state::GameState::Draw => 0,
        crate::game_state::GameState::Ongoing => {
            let mut best_score = if maximizing { i32::MIN } else { i32::MAX };

            for (row, col) in board.available_moves() {
                board.make_move(row, col, if maximizing { &Player::X } else { &Player::O });

                let child_score = alpha_beta_pruning(
                    board,
                    depth + 1,
                    alpha,
                    beta,
                    !maximizing,
                    cache,
                );

                board.grid[row][col] = None; // Undo the move.

                if maximizing {
                    best_score = best_score.max(child_score);
                    *alpha = (*alpha).max(best_score);
                } else {
                    best_score = best_score.min(child_score);
                    *beta = (*beta).min(best_score);
                }

                if *beta <= *alpha {
                    break; // Prune this branch.
                }
            }

            best_score
        }
    };

    cache.map.insert(hash, score);
    score
}

/// Determines the best move for the AI using Alpha-Beta Pruning.
/// 
/// # Arguments
/// * `board` - A mutable reference to the game board.
/// * `player` - A reference to the current player making the move.
/// * `cache` - A mutable reference to an `AlphaBetaCache` for storing previously evaluated states.
/// 
/// # Returns
/// A tuple `(usize, usize)` representing the row and column of the best move.
pub fn best_alpha_beta_move(
    board: &mut Board,
    player: &Player,
    cache: &mut AlphaBetaCache,
) -> (usize, usize) {
    let mut best_score = i32::MIN;
    let mut move_to_make = (0, 0);

    let mut alpha = i32::MIN;
    let mut beta = i32::MAX;

    println!("Evaluating possible moves with Alpha-Beta Pruning:");

    for (row, col) in board.available_moves() {
        board.make_move(row, col, player);
        let score = alpha_beta_pruning(
            board,
            0,
            &mut alpha,
            &mut beta,
            player == &Player::O,
            cache,
        );
        board.grid[row][col] = None; // Undo the move.

        println!("Move ({}, {}): Score = {}", row, col, score);

        if score > best_score {
            best_score = score;
            move_to_make = (row, col);
        }
    }

    println!(
        "Chosen move: ({}, {}) with score: {}",
        move_to_make.0, move_to_make.1, best_score
    );

    move_to_make
}
