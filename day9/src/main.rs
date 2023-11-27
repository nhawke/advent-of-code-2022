use std::cmp;
use std::ops::Sub;
use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut short_rope = Rope::new(2);
    let mut short_tail_positions = HashSet::new();
    let mut long_rope = Rope::new(10);
    let mut long_tail_positions = HashSet::new();
    for line in input.lines() {
        let (direction, amount) = line.split_once(" ").unwrap();
        let direction = Direction::new(direction);
        let amount: usize = amount.parse().unwrap();

        for _ in 0..amount {
            short_rope.do_move(direction);
            short_tail_positions.insert(short_rope.tail_position());
            long_rope.do_move(direction);
            long_tail_positions.insert(long_rope.tail_position());
        }
    }

    println!(
        "count of tail positions\nshort (2): {}\nlong (10): {}",
        short_tail_positions.len(),
        long_tail_positions.len()
    );
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
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
    knots: Vec<Vec2>,
}

impl Rope {
    fn new(len: usize) -> Rope {
        let mut knots: Vec<Vec2> = Vec::with_capacity(len);
        for _ in 0..len {
            knots.push(Vec2 { x: 0, y: 0 });
        }
        Rope { knots: knots }
    }

    fn tail_position(&self) -> Vec2 {
        self.knots[self.knots.len() - 1]
    }

    fn do_move(&mut self, dir: Direction) {
        // Move head.
        let head = &mut self.knots[0];
        match dir {
            Direction::Up => head.y += 1,
            Direction::Down => head.y -= 1,
            Direction::Left => head.x -= 1,
            Direction::Right => head.x += 1,
        }

        // Move tails to touch the previous knot if necessary.
        for i in 1..self.knots.len() {
            let prev = self.knots[i - 1];
            let tail = &mut self.knots[i];
            let Vec2 { x: dx, y: dy } = prev - *tail;
            if dx.abs() <= 1 && dy.abs() <= 1 {
                return;
            }

            match (dx, dy) {
                (0, 2) => tail.y += 1,
                (0, -2) => tail.y -= 1,
                (2, 0) => tail.x += 1,
                (-2, 0) => tail.x -= 1,
                (2, 1) | (1, 2) | (2, 2) => {
                    tail.x += 1;
                    tail.y += 1;
                }
                (2, -1) | (1, -2) | (2, -2) => {
                    tail.x += 1;
                    tail.y -= 1;
                }
                (-2, 1) | (-1, 2) | (-2, 2) => {
                    tail.x -= 1;
                    tail.y += 1;
                }
                (-2, -1) | (-1, -2) | (-2, -2) => {
                    tail.x -= 1;
                    tail.y -= 1;
                }
                _ => panic!(
                    "this shouldn't happen. knots {} ({:?}) and {} ({:?}) difference ({}, {})",
                    i - 1,
                    prev,
                    i,
                    tail,
                    dx,
                    dy
                ),
            }
        }
    }

    #[allow(dead_code)]
    fn string_representation(&self) -> String {
        let max_dimension = self
            .knots
            .iter()
            .map(|item| cmp::max(item.x.abs(), item.y.abs()))
            .max()
            .unwrap();

        let mut rows: Vec<Vec<char>> = Vec::new();

        for _ in -max_dimension..=max_dimension {
            let mut row: Vec<char> = Vec::new();
            for _ in -max_dimension..=max_dimension {
                row.push('.');
            }
            rows.push(row);
        }

        for (i, knot) in self.knots.iter().enumerate().rev() {
            let knot_char = if i == 0 {
                'H'
            } else {
                char::from_digit(i.try_into().unwrap(), 10).unwrap()
            };

            let y: usize = (max_dimension - knot.y).try_into().unwrap();
            let x: usize = (knot.x + max_dimension).try_into().unwrap();

            rows[y][x] = knot_char;
        }

        rows.iter()
            .map(|row| row.into_iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
