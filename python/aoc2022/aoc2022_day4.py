from __future__ import annotations

from pathlib import Path

class Range:
    def __init__(self, lower: int, upper: int) -> None:
        self.lower = lower
        self.upper = upper
    
    def contains(self, other: Range) -> bool:
        return other.lower >= self.lower and other.upper <= self.upper
    
    def overlaps(self, other: Range) -> bool:
        return not (other.lower > self.upper or other.upper < self.lower)

def main(path: Path):
    score = 0
    score_two = 0
    with path.open("r") as stream:
        for line in map(lambda x: x.strip(), stream):
            segments = line.split(",")
            elf_one, elf_two = [x.split("-") for x in segments]
            elf_one, elf_two = (
                Range(int(elf_one[0]), int(elf_one[1])),
                Range(int(elf_two[0]), int(elf_two[1]))
            )
            score += int(elf_one.contains(elf_two) or elf_two.contains(elf_one))
            score_two += int(elf_one.overlaps(elf_two) or elf_two.overlaps(elf_one))
    
    print(f"The tasks contain each other {score} times.")
    print(f"The tasks overlap {score_two} times.")


if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("input_file")

    args = parser.parse_args()

    path = Path(args.input_file)
    assert path.exists(), "Specified path does not exists."
    assert path.is_file(), "Specified path is not a file."
    main(path)
