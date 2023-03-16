use std::collections::VecDeque;

use crate::{direction::Direction, point::Point};

pub struct Snake {
    pub body: VecDeque<Point>,
    pub direction: Direction,
    growing: bool,
}

impl Snake {
    pub fn new(point: Point, lenght: u16, direction: Direction) -> Self {
        let oposite = direction.get_oposite();

        let mut body: VecDeque<Point> = VecDeque::new();
        body.push_back(point.clone());

        for count in 1..lenght {
            body.push_back(point.get_transformed(&oposite, count))
        }

        Self {
            body,
            direction,
            growing: false,
        }
    }

    pub fn slide(&mut self) {
        // Тело представляет собой очередь, и при движении мы удаляем последний элемент и прибавляем новый в зависимости от направления
        let head_point = self.body.get(0).unwrap();

        let new_point = head_point.get_transformed(&self.direction, 1);

        self.body.push_front(new_point);

        if !self.growing {
            self.body.pop_back();
        } else {
            self.growing = false;
        }
    }

    pub fn get_next_point(&self) -> Point {
        let head = self.body.get(0).expect("head must exist");
        head.get_transformed(&self.direction, 1)
    }

    pub fn grow(&mut self) {
        self.growing = true;
    }

    // сеттер для смены направления
    pub fn direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
}
