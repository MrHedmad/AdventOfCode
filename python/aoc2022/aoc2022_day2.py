from enum import Enum
from pathlib import Path

class Move(Enum):
    ROCK = 1
    PAPER = 2
    SCISSORS = 3

class Outcome(Enum):
    WIN = 6
    LOSE = 0
    DRAW = 3

conversions = {
    "A": Move.ROCK,
    "B": Move.PAPER,
    "C": Move.SCISSORS,
    "X": Move.ROCK,
    "Y": Move.PAPER,
    "Z": Move.SCISSORS
}

elf_conversions = {
    "A": Move.ROCK,
    "B": Move.PAPER,
    "C": Move.SCISSORS,
    "X": Outcome.LOSE,
    "Y": Outcome.DRAW,
    "Z": Outcome.WIN
}


def play(me: Move, you: Move) -> Outcome:
    if me == you:
        return Outcome.DRAW

    match (me, you):
        case (Move.ROCK, Move.PAPER):
            return Outcome.LOSE
        case (Move.ROCK, Move.SCISSORS):
            return Outcome.WIN
        case (Move.PAPER, Move.SCISSORS):
            return Outcome.LOSE
        case (Move.PAPER, Move.ROCK):
            return Outcome.WIN
        case (Move.SCISSORS, Move.ROCK):
            return Outcome.LOSE
        case (Move.SCISSORS, Move.PAPER):
            return Outcome.WIN
        case (x, y):
            raise ValueError(f"Invalid input values {x} vs {y}.")


def get_move_from_outcome(you: Move, outcome: Outcome) -> Move:
    if outcome == Outcome.DRAW:
        return you
    
    match (you, outcome):
        case (Move.ROCK, Outcome.WIN):
            return Move.PAPER
        case (Move.PAPER, Outcome.WIN):
            return Move.SCISSORS
        case (Move.SCISSORS, Outcome.WIN):
            return Move.ROCK
        case (Move.ROCK, Outcome.LOSE):
            return Move.SCISSORS
        case (Move.PAPER, Outcome.LOSE):
            return Move.ROCK
        case (Move.SCISSORS, Outcome.LOSE):
            return Move.PAPER
        case (x, y):
            raise ValueError(f"Invalid input values {x}, {y}.")


def main(path: Path):
    score = 0
    with path.open("r") as stream:
        for line in stream:
            you, me = line.split()
            me, you = conversions[me], conversions[you]

            score += play(me, you).value + me.value
    
    print(f"With my strategy, I'd get {score} points.")

    score = 0
    with path.open("r") as stream:
        for line in stream:
            you, outcome = line.split()
            you, outcome = elf_conversions[you], elf_conversions[outcome]

            score += get_move_from_outcome(you, outcome).value + outcome.value
    
    print(f"With the elf's strategy, I'd get {score} points.")


if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("input_file")

    args = parser.parse_args()

    path = Path(args.input_file)
    assert path.exists(), "Specified path does not exists."
    assert path.is_file(), "Specified path is not a file."
    main(path)
