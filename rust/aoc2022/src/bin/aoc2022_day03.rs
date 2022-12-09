use clap::Parser;
use std::path::PathBuf;
use color_eyre::Result;
use std::fs;

#[derive(Parser)]
struct Cli {
    input: Option<PathBuf>
}

#[derive(Debug)]
struct Rucksack {
    first_compartment: Compartment,
    second_compartment: Compartment
}

#[derive(Debug)]
struct Compartment {
    items: Vec<char>
}

impl Compartment {
    fn from_string(input: String) -> Compartment{
        Compartment { items: input.chars().collect() }
    }
}

impl Rucksack {
    fn from_string(input: String) -> Rucksack {
        let split = input.split_at(input.chars().count() / 2);
        Rucksack {
            first_compartment: Compartment::from_string(split.0.to_string()),
            second_compartment: Compartment::from_string(split.1.to_string())
        }
    }

    fn find_duplicates(&self) -> &char {

    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let path = cli.input.expect("You must supply a path");
    let contents = fs::read_to_string(path).expect("Failed to read into string");

    for line in contents.split("\n") {
        if line.is_empty() {
            continue;
        }

        let sack = Rucksack::from_string(line.to_owned());

        println!("Sack: {:?}", sack);
    }

    Ok(())
}
