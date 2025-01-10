/// Represents the possible states of the tic-tac-toe game.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GameState {
    /// Indicates that the game is still ongoing.
    Ongoing,
    /// Indicates a draw where no more moves are possible and there is no winner.
    Draw,
    /// Indicates that a player has won the game.
    Win(Player),
}

/// Represents the players in the tic-tac-toe game.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Player {
    /// Player X.
    X,
    /// Player O.
    O,
}

impl Player {
    /// Returns the opponent of the current player.
    /// 
    /// # Example
    /// ```
    /// let player = Player::X;
    /// assert_eq!(player.opponent(), Player::O);
    /// ```
    pub fn opponent(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}