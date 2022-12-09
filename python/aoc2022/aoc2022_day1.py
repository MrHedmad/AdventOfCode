#!usr/bin/env python
from pathlib import Path


def main(path: Path):
    calories = []
    elf = []
    with path.open("r") as stream:
        for line in stream:
            line = line.strip()
            if line == "":
                calories.append(elf)
                elf = []
                continue
            elf.append(int(line))
 
    totals = [sum(x) for x in calories]
    totals.sort(reverse=True)

    print(f"The elf with the most calories is carrying {totals[0]} calories.")
    print("The top three elves are carrying {} total calories.".format(sum(totals[:3])))


if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("input_file")

    args = parser.parse_args()

    path = Path(args.input_file)
    assert path.exists(), "Specified path does not exists."
    assert path.is_file(), "Specified path is not a file."
    main(path)
