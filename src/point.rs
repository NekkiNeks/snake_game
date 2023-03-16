use crate::direction::Direction;
use crate::snake::Snake;
use rand::{self, Rng};
use std::clone::Clone;

#[derive(Clone, PartialEq, Debug)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn get_transformed(&self, direction: &Direction, count: u16) -> Self {
        let mut new_point = self.clone();
        match direction {
            Direction::Down => new_point.y += count,
            Direction::Up => new_point.y -= count,
            Direction::Left => new_point.x -= count,
            Direction::Right => new_point.x += count,
        };

        new_point
    }

    pub fn get_random(width: u16, height: u16, snake: &Snake) -> Self {
        loop {
            let random_x = rand::thread_rng().gen_range(1, width - 1);
            let random_y = rand::thread_rng().gen_range(1, height - 1);

            let new_point = Point::new(random_x, random_y);

            if !snake.body.contains(&new_point) {
                break new_point;
            }
        }
    }
}
