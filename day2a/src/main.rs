use core::panic;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let input = fs::read_to_string(file_path).expect("file not found");

    let mut total_score: u32 = 0;
    for line in input.lines() {
        let (opponent_move, my_move) = line.split_once(" ").unwrap();
        let opponent_move = Move::pasre(opponent_move);
        let my_move = Move::pasre(my_move);

        total_score += play(my_move, opponent_move);
    }
    println!("{}", total_score);
}

fn play(my_move: Move, opponent_move: Move) -> u32 {
    let mut score = my_move.clone() as u32;

    score += match (my_move, opponent_move) {
        (x, y) if x == y => Outcome::Draw,
        (Move::Rock, Move::Paper)
        | (Move::Paper, Move::Scissors)
        | (Move::Scissors, Move::Rock) => Outcome::Lose,
        _ => Outcome::Win,
    } as u32;
    return score;
}

#[repr(u32)]
enum Outcome {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

#[derive(Clone, PartialEq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    fn pasre(move_string: &str) -> Move {
        match move_string {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!("invalid move {}", move_string),
        }
    }
}
