from pathlib import Path

def main():
    pass

if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("input_file")

    args = parser.parse_args()

    path = Path(args.input_file)
    assert path.exists(), "Specified path does not exists."
    assert path.is_file(), "Specified path is not a file."
    main(path)
