use clap::Parser;
use std::path::PathBuf;
use std::fs;

#[derive(Parser)]
struct Cli {
    input: Option<PathBuf>
}

fn get_rps(input: &str) -> Result<String, String> {
    match input {
        "A" | "X" => Ok(String::from("Rock")),
        "B" | "Y" => Ok(String::from("Paper")),
        "C" | "Z" => Ok(String::from("Scissors")),
        _ => Err(String::from("Invalid Input"))
    }
}

fn get_score(you: &str, me: &str) -> u64 {
    let you = get_rps(you).expect("Invalid you");
    let me = get_rps(me).expect("Invalid me");

    let win: u64 = if (you == "Rock") & (me == "Paper") {
        6
    } else if (you == "Rock") & (me == "Scissors") {
        0
    } else if (you == "Paper") & (me == "Scissors") {
        6
    } else if (you == "Paper") & (me == "Rock") {
        0
    } else if (you == "Scissors") & (me == "Rock") {
        6
    } else if (you == "Scissors") & (me == "Paper") {
        0
    } else {
        // It's a draw, since the only remaining cases are if the two are equal
        3
    };

    let my_score: u64 = match &me[..] {
        "Rock" => 1,
        "Paper" => 2,
        "Scissors" => 3,
        _ => panic!("Invalid me input.")
    };

    win + my_score
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = Cli::parse();

    let path = cli.input.expect("You must supply a path");

    let contents = fs::read_to_string(path)?;
    let lines = contents.split("\n");

    let mut scores: Vec<u64> = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }

        let values: Vec<&str> = line.split_whitespace().collect();
        scores.push(get_score(values[0], values[1]))
    }

    println!("{:?}", scores);

    println!("Your total score would be {}", scores.iter().sum::<u64>());

    Ok(())
} 