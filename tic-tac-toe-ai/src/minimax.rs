use crate::board::Board;
use crate::game_state::Player;
use std::collections::HashMap;

pub fn minimax(
    board: &mut Board,
    depth: i32,
    maximizing: bool,
    cache: &mut HashMap<u64, i32>,
) -> i32 {
    let hash = board.hash_state();
    if let Some(&cached_score) = cache.get(&hash) {
        return cached_score;
    }

    match board.get_winner() {
        crate::game_state::GameState::Win(Player::X) => return 10 - depth,
        crate::game_state::GameState::Win(Player::O) => return depth - 10,
        crate::game_state::GameState::Draw => return 0,
        crate::game_state::GameState::Ongoing => (),
    }

    let mut best_score = if maximizing { i32::MIN } else { i32::MAX };
    for (row, col) in board.available_moves() {
        board.make_move(row, col, if maximizing { Player::X } else { Player::O });
        let score = minimax(board, depth + 1, !maximizing, cache);
        board.grid[row][col] = None; // Undo the move

        if maximizing {
            best_score = best_score.max(score);
        } else {
            best_score = best_score.min(score);
        }
    }

    cache.insert(hash, best_score);
    best_score
}

pub fn best_move(board: &mut Board, player: Player) -> (usize, usize) {
    let mut best_score = i32::MIN;
    let mut move_to_make = (0, 0);
    let mut cache = HashMap::new();

    for (row, col) in board.available_moves() {
        board.make_move(row, col, player.clone());
        let score = minimax(board, 0, player == Player::O, &mut cache);
        board.grid[row][col] = None; // Undo the move

        if score > best_score {
            best_score = score;
            move_to_make = (row, col);
        }
    }

    move_to_make
}
