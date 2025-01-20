pub mod board;
pub mod game_state;
pub mod user_interface;

pub mod cache {
    pub mod cache_minimax;
    pub mod cache_minimax_weighted;
    pub mod cache_minimax_alpha_beta_pruning;
}

pub mod minimax {
    pub mod minimax;
    pub mod minimax_weighted;
    pub mod minimax_alpha_beta_pruning;
}
