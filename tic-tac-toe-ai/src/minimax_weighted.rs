use crate::board::Board;
use crate::game_state::Player;
use crate::knowledge_cache::WeightedStateEvaluation;
use std::collections::HashMap;

/// Board weights used to guide the weighted minimax algorithm.
/// 
/// These weights prioritize strategic positions on the tic-tac-toe board:
/// - The center (weight 5) is the most strategic position, as it provides control over multiple lines.
/// - Corners (weight 3) are valuable for forming winning combinations and blocking opponents.
/// - Edges (weight 2) are less critical but can still contribute to winning lines.
/// 
/// This heuristic guides the AI to make moves that are more likely to result in a win or block the opponent effectively.
const WEIGHTS: [[i32; 3]; 3] = [
    [3, 2, 3],
    [2, 5, 2],
    [3, 2, 3],
];

pub fn best_weighted_move(
    board: &mut Board,
    player: Player,
    cache: &mut HashMap<u64, WeightedStateEvaluation>,
) -> (usize, usize) {
    let mut best_score = i32::MIN;
    let mut move_to_make = (0, 0);

    println!("Evaluating possible moves:");

    for (row, col) in board.available_moves() {
        board.make_move(row, col, player.clone());
        let score = weighted_minimax(board, 0, player == Player::O, cache);
        board.grid[row][col] = None;

        // Debug output for decision evaluation
        println!(
            "Move ({}, {}): Score = {}, Weight = {}",
            row, col, score, WEIGHTS[row][col]
        );

        // Combine score and weight with scaling factor
        let weighted_score = score + (WEIGHTS[row][col] * 10); // Scale weights by 10

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

fn weighted_minimax(
    board: &mut Board,
    depth: i32,
    maximizing: bool,
    cache: &mut HashMap<u64, WeightedStateEvaluation>,
) -> i32 {
    let hash = board.hash_state();
    if let Some(cached_eval) = cache.get(&hash) {
        return cached_eval.score;
    }

    let score = match board.get_winner() {
        crate::game_state::GameState::Win(Player::X) => 10 - depth,
        crate::game_state::GameState::Win(Player::O) => depth - 10,
        crate::game_state::GameState::Draw => 0,
        crate::game_state::GameState::Ongoing => {
            let mut best_score = if maximizing { i32::MIN } else { i32::MAX };

            for (row, col) in board.available_moves() {
                board.make_move(row, col, if maximizing { Player::X } else { Player::O });
                let child_score = weighted_minimax(board, depth + 1, !maximizing, cache);
                board.grid[row][col] = None;

                // Add weight influence with scaling
                let position_weight = WEIGHTS[row][col] * 10; // Scale by 10
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

    cache.insert(
        hash,
        WeightedStateEvaluation {
            score,
            weights: evaluate_weights(board),
        },
    );
    score
}

pub fn evaluate_weights(board: &Board) -> Vec<i32> {
    let mut flat_weights = vec![0; 9];
    for (i, row) in WEIGHTS.iter().enumerate() {
        for (j, &weight) in row.iter().enumerate() {
            if board.grid[i][j].is_some() {
                flat_weights[i * 3 + j] = weight;
            }
        }
    }
    flat_weights
}
