use std::ops::Sub;
use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut rope = Rope::new();
    let mut tail_positions = HashSet::new();
    for line in input.lines() {
        let (direction, amount) = line.split_once(" ").unwrap();
        let direction = Direction::new(direction);
        let amount: usize = amount.parse().unwrap();

        for _ in 0..amount {
            rope.do_move(direction);
            tail_positions.insert(rope.tail);
        }
    }

    println!("count of tail positions: {}", tail_positions.len());
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new(dir: &str) -> Direction {
        match dir {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "R" => Direction::Right,
            "L" => Direction::Left,
            _ => panic!("invalid direction {}", dir),
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

struct Rope {
    head: Vec2,
    tail: Vec2,
}

impl Rope {
    fn new() -> Rope {
        Rope {
            head: Vec2 { x: 0, y: 0 },
            tail: Vec2 { x: 0, y: 0 },
        }
    }
    fn do_move(&mut self, dir: Direction) {
        // Move head.
        match dir {
            Direction::Up => self.head.y += 1,
            Direction::Down => self.head.y -= 1,
            Direction::Left => self.head.x -= 1,
            Direction::Right => self.head.x += 1,
        }

        // Move tail to touch head if necessary.
        let Vec2 { x: dx, y: dy } = self.head - self.tail;
        if dx.abs() <= 1 && dy.abs() <= 1 {
            return;
        }

        match (dx, dy) {
            (0, 2) => self.tail.y += 1,
            (0, -2) => self.tail.y -= 1,
            (2, 0) => self.tail.x += 1,
            (-2, 0) => self.tail.x -= 1,
            (2, 1) | (1, 2) => {
                self.tail.x += 1;
                self.tail.y += 1;
            }
            (2, -1) | (1, -2) => {
                self.tail.x += 1;
                self.tail.y -= 1;
            }
            (-2, 1) | (-1, 2) => {
                self.tail.x -= 1;
                self.tail.y += 1;
            }
            (-2, -1) | (-1, -2) => {
                self.tail.x -= 1;
                self.tail.y -= 1;
            }
            _ => panic!("this shouldn't happen. difference ({}, {})", dx, dy),
        }
    }
}
