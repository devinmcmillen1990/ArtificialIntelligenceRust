mod board;
mod minimax;
mod minimax_weighted;
mod game_state;
mod knowledge_cache;
mod user_interface;

use board::Board;
use game_state::{GameState, Player};
use knowledge_cache::{MinimaxCache, WeightedCache};
use user_interface::{display_board, get_user_move, parse_command_line_args};

fn main() {
    // Parse command line arguments to determine the algorithm to use
    let use_weighted = parse_command_line_args();

    // Load the appropriate cache
    if use_weighted {
        let mut cache = WeightedCache::load_from_file("cache_weighted.json");
        println!("Loaded {} states from weighted cache.", cache.map.len());

        let mut board = Board::new();
        let mut current_player = Player::X;

        println!("You are Player X and will make the first move!");

        loop {
            display_board(&board);
            match board.get_winner() {
                GameState::Win(player) => {
                    println!("{:?} wins!", player);
                    break;
                }
                GameState::Draw => {
                    println!("It's a draw!");
                    break;
                }
                GameState::Ongoing => (),
            }

            if current_player == Player::X {
                loop {
                    let (row, col) = get_user_move();
                    if board.make_move(row, col, current_player) {
                        break;
                    } else {
                        println!("Invalid move. Try again.");
                    }
                }
            } else {
                let (row, col) =
                    minimax_weighted::best_weighted_move(&mut board, current_player, &mut cache.map);
                board.make_move(row, col, current_player);
                cache.save_to_file("cache_weighted.json");
            }

            current_player = if current_player == Player::X {
                Player::O
            } else {
                Player::X
            };
        }

        cache.save_to_file("cache_weighted.json");
    } else {
        let mut cache = MinimaxCache::load_from_file("cache_minimax.json");
        println!("Loaded {} states from minimax cache.", cache.map.len());

        let mut board = Board::new();
        let mut current_player = Player::X;

        println!("You are Player X and will make the first move!");

        loop {
            display_board(&board);
            match board.get_winner() {
                GameState::Win(player) => {
                    println!("{:?} wins!", player);
                    break;
                }
                GameState::Draw => {
                    println!("It's a draw!");
                    break;
                }
                GameState::Ongoing => (),
            }

            if current_player == Player::X {
                loop {
                    let (row, col) = get_user_move();
                    if board.make_move(row, col, current_player) {
                        break;
                    } else {
                        println!("Invalid move. Try again.");
                    }
                }
            } else {
                let (row, col) =
                    minimax::best_move(&mut board, current_player, &mut cache.map);
                board.make_move(row, col, current_player);
                cache.save_to_file("cache_minimax.json");
            }

            current_player = if current_player == Player::X {
                Player::O
            } else {
                Player::X
            };
        }

        cache.save_to_file("cache_minimax.json");
    }
}
