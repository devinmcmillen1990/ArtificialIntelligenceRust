use tic_tac_toe::{
    board::Board,
    game_state::Player,
    cache::{
        cache_minimax::MinimaxCache,
        cache_minimax_weighted::WeightedCache,
        cache_minimax_alpha_beta_pruning::AlphaBetaCache,
    },
    minimax::{
        minimax,
        minimax_weighted,
        minimax_alpha_beta_pruning,
    },
    user_interface::{parse_command_line_args, get_user_move},
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = parse_command_line_args(&args);

    let mut board = Board::new();

    // Initialize caches
    let mut minimax_cache = MinimaxCache::load_from_file("minimax_cache.json");
    let mut weighted_cache = WeightedCache::load_from_file("weighted_cache.json");
    let mut alpha_beta_cache = AlphaBetaCache::load_from_file("alpha_beta_cache.json");

    println!("You are Player X and will make the first move!");

    loop {
        board.display();
        if let Some(player_move) = get_user_move(&board) {
            board.make_move(player_move.0, player_move.1, &Player::X);
        } else {
            println!("Invalid move! Try again.");
            continue;
        }

        if let Some(result) = board.get_winner() {
            println!("{}", result);
            break;
        }

        let ai_move = match mode.as_str() {
            "--w" | "-w" => {
                minimax_weighted::best_weighted_move(&mut board, &Player::O, &mut weighted_cache.map)
            }
            "--ab-pruning" | "--a" => {
                minimax_alpha_beta_pruning::best_alpha_beta_move(&mut board, &Player::O, &mut alpha_beta_cache)
            }
            _ => {
                minimax::best_move(&mut board, Player::O, &mut minimax_cache.map)
            }
        };

        board.make_move(ai_move.0, ai_move.1, &Player::O);

        if let Some(result) = board.get_winner() {
            println!("{}", result);
            break;
        }
    }

    // Save caches to files
    minimax_cache.save_to_file("cache_minimax.json");
    weighted_cache.save_to_file("cache_minimax_weighted.json");
    alpha_beta_cache.save_to_file("cache_alpha_beta.json");

    println!("Game Over!");
}
