pub mod action;
pub mod direction;
pub mod game;
pub mod point;
pub mod snake;

fn main() {
    game::Game::new(std::io::stdout(), 40, 20).run();
}
