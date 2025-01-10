mod board;
mod minimax;
mod game_state;
mod user_interface;

use board::Board;
use minimax::best_move;
use game_state::{GameState, Player};
use user_interface::{display_board, get_user_move, print_game_result};

fn main() {
    let mut board = Board::new();
    let mut current_player = Player::X;

    loop {
        display_board(&board);
        match board.get_winner() {
            GameState::Win(player) => {
                print_game_result(GameState::Win(player));
                break;
            }
            GameState::Draw => {
                print_game_result(GameState::Draw);
                break;
            }
            GameState::Ongoing => (),
        }

        if current_player == Player::X {
            let (row, col) = best_move(&mut board, current_player);
            board.make_move(row, col, current_player);
        } else {
            let (row, col) = get_user_move();
            if !board.make_move(row, col, current_player) {
                println!("Invalid move. Try again.");
                continue;
            }
        }

        current_player = if current_player == Player::X {
            Player::O
        } else {
            Player::X
        };
    }
}
