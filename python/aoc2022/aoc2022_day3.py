from pathlib import Path
from string import ascii_letters
import more_itertools
    
def items_in_common(*args) -> list[str]:
    assert len(args), "You must supply at least two iterables."
    candidates = set(args[0])
    search_space = [set(item_set) for item_set in args[1:]]
    common = []
    for c in candidates:
        if all([c in x for x in search_space]):
            common.append(c)
    
    return common

def split_in_half(string: str) -> tuple[str, str]:
    assert len(string) % 2 == 0, "Invalid contents length"
    half = len(string) // 2
    return (string[:half], string[half:])

def main(path: Path):
    score = 0
    badge_score = 0
    with path.open("r") as stream:
        for line in map(lambda x: x.strip(), stream):
            score += ascii_letters.index(items_in_common(*split_in_half(line))[0]) + 1
        
        stream.seek(0)
        for lines in more_itertools.grouper(map(lambda x: x.strip(), stream), 3, incomplete="strict"):
            badge_score += ascii_letters.index(items_in_common(*lines)[0]) + 1

    print(f"The sum of the items is common is {score}.")
    print(f"The sum of the badges is {badge_score}.")
    

if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("input_file")

    args = parser.parse_args()

    path = Path(args.input_file)
    assert path.exists(), "Specified path does not exists."
    assert path.is_file(), "Specified path is not a file."
    main(path)
