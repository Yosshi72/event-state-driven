#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum State {
    Normal,
    Super,
    Fire,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum InvincibleState {
    Starred,
    NonStarred,
}