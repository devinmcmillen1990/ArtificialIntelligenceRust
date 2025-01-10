use std::env;
use crate::board::Board;
use std::io;

/// Displays the current state of the game board.
/// 
/// Iterates over the board's grid and prints each cell's state:
/// - `X` or `O` for occupied cells.
/// - `.` for empty cells.
/// 
/// # Parameters
/// - `board`: A reference to the current `Board` to be displayed.
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
/// 
/// Displays a prompt and waits for the user to input their move as two space-separated integers.
/// Parses the input into a tuple `(row, col)`.
/// 
/// # Returns
/// A tuple `(usize, usize)` representing the row and column of the user's chosen move.
/// 
/// # Panics
/// - If the input cannot be parsed into two integers.
/// - If the input format is incorrect.
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
/// 
/// Scans the arguments for either `-weighted` or `--w`.
/// 
/// # Returns
/// - `true` if the weighted algorithm is specified.
/// - `false` otherwise.
/// 
/// # Examples
/// ```
/// // If the program is run with `--w`:
/// assert_eq!(parse_command_line_args(), true);
/// ```
pub fn parse_command_line_args() -> bool {
    let args: Vec<String> = env::args().collect();
    args.iter().any(|arg| arg == "-weighted" || arg == "--w")
}
