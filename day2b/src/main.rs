use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let input = fs::read_to_string(file_path).expect("file not found");

    let mut total_score: u32 = 0;
    for line in input.lines() {
        let (opponent_move, my_outcome) = line.split_once(" ").unwrap();
        let opponent_move = Move::pasre(opponent_move);
        let my_outcome = Outcome::parse(my_outcome);

        total_score += play_with_outcome(my_outcome, opponent_move);
    }
    println!("{}", total_score);
}

fn play_with_outcome(my_outcome: Outcome, opponent_move: Move) -> u32 {
    let mut score: u32 = my_outcome.clone() as u32;

    score += match (my_outcome, opponent_move) {
        (Outcome::Draw, opp) => opp,
        (Outcome::Win, Move::Scissors) | (Outcome::Lose, Move::Paper) => Move::Rock,
        (Outcome::Win, Move::Rock) | (Outcome::Lose, Move::Scissors) => Move::Paper,
        (Outcome::Win, Move::Paper) | (Outcome::Lose, Move::Rock) => Move::Scissors,
    } as u32;
    return score;
}

#[derive(Clone)]
#[repr(u32)]
enum Outcome {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

impl Outcome {
    fn parse(outcome: &str) -> Outcome {
        match outcome {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("invalid outcome {}", outcome),
        }
    }
}

#[derive(Clone)]
#[repr(u32)]
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
