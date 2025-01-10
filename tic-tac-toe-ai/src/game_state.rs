#[derive(Clone, Copy, PartialEq, Hash, Eq, Debug)]
pub enum Player {
    X,
    O,
}

#[derive(Clone, PartialEq, Debug)]
pub enum GameState {
    Ongoing,
    Draw,
    Win(Player),
}
