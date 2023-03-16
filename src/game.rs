use std::{io::Stdout, time::Duration};

use crate::{action::Action, direction::Direction, point::Point, snake::Snake};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{enable_raw_mode, size, Clear, ClearType, SetSize},
    ErrorKind, ExecutableCommand,
};
use rand::Rng;

const SNAKE_CHAR: char = 'S';
const WALL_CHAR: char = '#';
const FOOD_CHAR: char = '•';
const BG_CHAR: char = ' ';

pub struct Game {
    stdout: Stdout,
    original_terminal_size: (u16, u16),
    width: u16,
    height: u16,
    food: Point,
    snake: Snake,
    score: u16,
}

impl Game {
    pub fn new(stdout: Stdout, width: u16, height: u16) -> Self {
        let original_terminal_size = size().unwrap();

        let starting_point = Point::new(width / 2, height / 2);
        let starting_length = 3;
        let starting_direction = match rand::thread_rng().gen_range(0, 4) {
            0 => Direction::Down,
            1 => Direction::Left,
            2 => Direction::Right,
            _ => Direction::Up,
        };

        let snake = Snake::new(starting_point, starting_length, starting_direction);

        let food = Point::get_random(width, height, &snake);

        Game {
            stdout,
            original_terminal_size,
            width,
            height,
            food,
            snake,
            score: 0,
        }
    }

    pub fn run(&mut self) {
        self.prepare_screen().unwrap();

        let input_duration = std::time::Duration::from_millis(500);

        loop {
            self.render();

            // обработка ввода пользователя
            if let Some(action) = self.get_user_action(input_duration) {
                match action {
                    Action::Quit => break,
                    Action::Move(direction) => {
                        if direction.get_oposite() != self.snake.direction {
                            self.snake.direction(direction);
                        }
                    }
                }
            }

            let next_point = self.snake.get_next_point();

            if self.is_a_wall(&next_point) {
                break;
            }

            if self.is_a_food(&next_point) {
                self.snake.grow();
                self.generate_food();
                self.score += 1;
            }

            self.snake.slide();
        }

        self.set_screen_to_default().unwrap()
    }

    // Обработка и проверка событий

    fn is_a_wall(&self, point: &Point) -> bool {
        point.x == 0 || point.x == self.width - 1 || point.y == 0 || point.y == self.height - 1
    }

    fn is_a_food(&self, point: &Point) -> bool {
        point == &self.food
    }

    // Функции

    fn generate_food(&mut self) {
        self.food = Point::get_random(self.width, self.height, &self.snake)
    }

    fn get_user_action(&self, duration: Duration) -> Option<Action> {
        let event_available = poll(duration).ok()?;

        if !event_available {
            return None;
        }

        let event = match read().ok()? {
            Event::Key(key_event) => key_event,
            _ => return None,
        };

        let action = match event.code {
            KeyCode::Up => Action::Move(Direction::Up),
            KeyCode::Down => Action::Move(Direction::Down),
            KeyCode::Left => Action::Move(Direction::Left),
            KeyCode::Right => Action::Move(Direction::Right),
            KeyCode::Char('q') | KeyCode::Char('й') => Action::Quit,
            _ => return None,
        };

        Some(action)
    }
    // Рендеринг

    fn render(&mut self) {
        self.draw_borders().unwrap();
        self.draw_background().unwrap();
        self.draw_snake().unwrap();
        self.draw_food().unwrap();
        self.draw_score().unwrap()
    }

    fn prepare_screen(&mut self) -> Result<(), ErrorKind> {
        enable_raw_mode()?;
        self.stdout
            .execute(SetSize(self.width, self.height))?
            .execute(Clear(ClearType::All))?
            .execute(Hide)?;

        Ok(())
    }

    fn set_screen_to_default(&mut self) -> Result<(), ErrorKind> {
        self.stdout
            .execute(Clear(ClearType::All))?
            .execute(SetSize(
                self.original_terminal_size.0,
                self.original_terminal_size.1,
            ))?
            .execute(Show)?
            .execute(ResetColor)?;

        Ok(())
    }

    fn draw_background(&mut self) -> Result<(), ErrorKind> {
        //set colors
        self.stdout
            .execute(SetBackgroundColor(Color::Black))
            .unwrap();

        for row_index in 1..(self.height - 1) {
            for col_index in 1..(self.width - 1) {
                self.stdout
                    .execute(MoveTo(col_index, row_index))?
                    .execute(Print(BG_CHAR))?;
            }
        }

        Ok(())
    }

    fn draw_borders(&mut self) -> Result<(), ErrorKind> {
        // set colors
        self.stdout
            .execute(SetBackgroundColor(Color::Rgb {
                r: 150,
                g: 150,
                b: 150,
            }))?
            .execute(SetForegroundColor(Color::Black))?;

        for row_index in 0..=self.height {
            if row_index == 0 || row_index == self.height {
                //draw line
                for col_index in 0..=self.width {
                    self.stdout
                        .execute(MoveTo(col_index, row_index))?
                        .execute(Print(WALL_CHAR))?;
                }
            } else {
                //draw sides
                //left
                self.stdout
                    .execute(MoveTo(0, row_index))?
                    .execute(Print(WALL_CHAR))?;

                //right
                self.stdout
                    .execute(MoveTo(self.width, row_index))?
                    .execute(Print(WALL_CHAR))?;
            }
        }

        Ok(())
    }

    fn draw_snake(&mut self) -> Result<(), ErrorKind> {
        //set colors
        self.stdout.execute(SetBackgroundColor(Color::White))?;
        self.stdout.execute(SetForegroundColor(Color::Black))?;

        for point in &self.snake.body {
            self.stdout
                .execute(MoveTo(point.x, point.y))?
                .execute(Print(SNAKE_CHAR))?;
        }

        Ok(())
    }

    fn draw_food(&mut self) -> Result<(), ErrorKind> {
        //set colors
        self.stdout
            .execute(SetBackgroundColor(Color::White))?
            .execute(SetForegroundColor(Color::Black))?;

        self.stdout
            .execute(MoveTo(self.food.x, self.food.y))?
            .execute(Print(FOOD_CHAR))?;

        Ok(())
    }

    fn draw_score(&mut self) -> Result<(), ErrorKind> {
        let string = format!("Score: {}", self.score);
        let position = (self.width / 2) - (string.len() as u16 / 2) - 1;

        self.stdout
            .execute(MoveTo(position, 0))?
            .execute(Print(string))?;

        Ok(())
    }
}
