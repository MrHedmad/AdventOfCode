use clap::Parser;
use std::path::PathBuf;
use std::fs;

#[derive(Parser)]
struct Cli {
    input: Option<PathBuf>
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = Cli::parse();

    let path = cli.input.expect("You must supply a path");

    let contents = fs::read_to_string(path)?;

    let lines = contents.split("\n");
    
    let mut elves: Vec<Vec<u64>> = Vec::new();

    let mut elf: Vec<u64> = Vec::new();
    for line in lines {
        if line.is_empty() {
            elves.push(elf.clone());
            elf.clear();
            continue;
        }

        elf.push(line.parse()?);
    }

    let mut calories_sum: Vec<u64> = elves.iter().map(|x: &Vec<u64>| {x.iter().sum::<u64>()}).collect();

    println!("The elf that carries the most calories is carrying {:?} calories.", calories_sum.iter().max().unwrap());

    // Second part of the problem
    calories_sum.sort_unstable();

    let top_three: &Vec<u64> = &calories_sum.into_iter().rev().collect();

    println!("The top three elves are carrying a total of {:?} calories.", &top_three[..3].into_iter().sum::<u64>());

    Ok(())
}
