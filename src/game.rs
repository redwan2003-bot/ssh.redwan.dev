use std::collections::VecDeque;

/// Direction the snake is currently moving.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

/// A position on the game grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    pub x: i16,
    pub y: i16,
}

/// State for the Snake mini-game.
#[derive(Debug, Clone)]
pub struct SnakeGame {
    pub width: u16,
    pub height: u16,
    pub snake: VecDeque<Pos>,
    pub direction: Direction,
    pub food: Pos,
    pub score: u16,
    pub game_over: bool,
    pub tick: usize,
    seed: u32,
}

impl SnakeGame {
    pub fn new(width: u16, height: u16) -> Self {
        let cx = (width / 2) as i16;
        let cy = (height / 2) as i16;
        let mut snake = VecDeque::new();
        snake.push_back(Pos { x: cx, y: cy });
        snake.push_back(Pos { x: cx - 1, y: cy });
        snake.push_back(Pos { x: cx - 2, y: cy });

        let mut game = Self {
            width,
            height,
            snake,
            direction: Direction::Right,
            food: Pos { x: 0, y: 0 },
            score: 0,
            game_over: false,
            tick: 0,
            seed: 42,
        };
        game.spawn_food();
        game
    }

    /// Simple LCG pseudo-random number generator (no external crate needed).
    fn next_rand(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1664525).wrapping_add(1013904223);
        self.seed
    }

    fn spawn_food(&mut self) {
        let max_attempts = 100;
        let mut attempts = 0;
        loop {
            attempts += 1;
            let x = (self.next_rand() % self.width as u32) as i16;
            let y = (self.next_rand() % self.height as u32) as i16;
            let pos = Pos { x, y };
            if !self.snake.contains(&pos) || attempts > max_attempts {
                self.food = pos;
                break;
            }
        }
    }

    pub fn change_direction(&mut self, dir: Direction) {
        if dir != self.direction.opposite() {
            self.direction = dir;
        }
    }

    /// Advance the game by one tick. Returns true if state changed.
    pub fn tick(&mut self) -> bool {
        if self.game_over {
            return false;
        }

        self.tick += 1;

        let head = self.snake.front().copied().unwrap();
        let new_head = match self.direction {
            Direction::Up => Pos { x: head.x, y: head.y - 1 },
            Direction::Down => Pos { x: head.x, y: head.y + 1 },
            Direction::Left => Pos { x: head.x - 1, y: head.y },
            Direction::Right => Pos { x: head.x + 1, y: head.y },
        };

        // Check wall collision
        if new_head.x < 0
            || new_head.y < 0
            || new_head.x >= self.width as i16
            || new_head.y >= self.height as i16
        {
            self.game_over = true;
            return true;
        }

        // Check self collision
        if self.snake.contains(&new_head) {
            self.game_over = true;
            return true;
        }

        self.snake.push_front(new_head);

        if new_head == self.food {
            self.score += 1;
            self.spawn_food();
        } else {
            self.snake.pop_back();
        }

        true
    }
}
