use crate::board::Board;
use crate::game_state::{GameState, Player};
use std::io;

pub fn display_board(board: &Board) {
    for row in &board.grid {
        for cell in row {
            match cell {
                Some(Player::X) => print!("X "),
                Some(Player::O) => print!("O "),
                None => print!(". "),
            }
        }
        println!();
    }
}

pub fn get_user_move() -> (usize, usize) {
    println!("Enter your move (row and col): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.trim().split_whitespace();
    let row: usize = parts.next().unwrap().parse().unwrap();
    let col: usize = parts.next().unwrap().parse().unwrap();
    (row, col)
}

pub fn print_game_result(game_state: GameState) {
    match game_state {
        GameState::Win(player) => println!("{:?} wins!", player),
        GameState::Draw => println!("It's a draw!"),
        GameState::Ongoing => (),
    }
}
