use crate::direction::Direction;

pub enum Action {
    Quit,
    Move(Direction),
}
