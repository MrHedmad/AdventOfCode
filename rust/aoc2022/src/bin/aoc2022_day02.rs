use clap::Parser;
use std::path::PathBuf;
use std::fs;

#[derive(Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors
}

impl Move {
    fn from_str(input: &str) -> Result<Move, String> {
        match input {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(String::from("Invalid Input"))
        }
    }

    fn from_outcome(&self, outcome: &str) -> Result<Move, String> {
        match (self, outcome) {
            // X > lose, Y > draw, Z > win
            (Move::Rock, "X") => Ok(Move::Scissors),
            (Move::Rock, "Z") => Ok(Move::Paper),
            (Move::Paper, "X") => Ok(Move::Rock),
            (Move::Paper, "Z") => Ok(Move::Scissors),
            (Move::Scissors, "X") => Ok(Move::Paper),
            (Move::Scissors, "Z") => Ok(Move::Rock),
            (_, "Y") => Ok(self.clone()),
            (_, _) => Err(String::from("Invalid input moves."))
        }
    }

    fn to_score(&self) -> u64 {
        match &self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3
        }
    }

    fn fight(&self, other: &Move) -> u64 {
        match (self, other) {
            (Move::Paper, Move::Rock) => 6,
            (Move::Paper, Move::Scissors) => 0,
            (Move::Rock, Move::Paper) => 0,
            (Move::Rock, Move::Scissors) => 6,
            (Move::Scissors, Move::Paper) => 6,
            (Move::Scissors, Move::Rock) => 0,
            (_, _) => 3
        }
    }
}

#[derive(Parser)]
struct Cli {
    input: Option<PathBuf>
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = Cli::parse();

    let path = cli.input.expect("You must supply a path");

    let contents = fs::read_to_string(path)?;
    let lines = contents.split("\n");

    let mut scores_naive: Vec<u64> = Vec::new();
    let mut scores_elf: Vec<u64> = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }

        let values: Vec<&str> = line.split_whitespace().collect();
        let your_move = Move::from_str(&values[0]).expect("Invalid move from you.");
        let my_naive_move = Move::from_str(&values[1]).expect("Invalid move from me.");
        let my_elf_move = your_move.from_outcome(&values[1]).expect("Invalid expectation");
        scores_naive.push(
            my_naive_move.fight(&your_move) + my_naive_move.to_score()
        );
 
        scores_elf.push(
            my_elf_move.fight(&your_move) + my_elf_move.to_score()
        );
    }
    println!("Your total score would be {} if you follow the naive schema.", scores_naive.iter().sum::<u64>());
    println!("Your total score would be {} if you follow the elf schema.", scores_elf.iter().sum::<u64>());

    Ok(())
} 