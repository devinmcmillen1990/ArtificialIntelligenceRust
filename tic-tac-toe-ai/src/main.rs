mod board;
mod minimax;
mod game_state;

use board::Board;
use minimax::best_move;
use game_state::Player;

fn main() {
    let mut board = Board::new();
    let mut current_player = Player::X;

    loop {
        board.display();
        match board.get_winner() {
            game_state::GameState::Win(player) => {
                println!("{:?} wins!", player);
                break;
            }
            game_state::GameState::Draw => {
                println!("It's a draw!");
                break;
            }
            game_state::GameState::Ongoing => (),
        }

        if current_player == Player::X {
            let (row, col) = best_move(&mut board, current_player);
            board.make_move(row, col, current_player);
        } else {
            println!("Enter your move (row and col): ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut parts = input.trim().split_whitespace();
            let row: usize = parts.next().unwrap().parse().unwrap();
            let col: usize = parts.next().unwrap().parse().unwrap();
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
