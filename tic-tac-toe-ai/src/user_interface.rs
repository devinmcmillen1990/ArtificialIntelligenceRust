use std::env;
use crate::board::Board;
use std::io;

/// Displays the current state of the game board.
pub fn display_board(board: &Board) {
    for row in &board.grid {
        for cell in row {
            match cell {
                Some(player) => print!("{:?} ", player),
                None => print!(". "),
            }
        }
        println!();
    }
}

/// Prompts the user to input their move.
pub fn get_user_move() -> (usize, usize) {
    println!("Enter your move (row and col): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.trim().split_whitespace();
    let row: usize = parts.next().unwrap().parse().unwrap();
    let col: usize = parts.next().unwrap().parse().unwrap();
    (row, col)
}

/// Parses command line arguments to determine if the weighted algorithm should be used.
pub fn parse_command_line_args() -> bool {
    let args: Vec<String> = env::args().collect();
    args.iter().any(|arg| arg == "-weighted" || arg == "--w")
}
