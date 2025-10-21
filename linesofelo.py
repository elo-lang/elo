# simple joke script that counts the lines of the Pile's implementation code
from os.path import join, isdir
from os import listdir

SOURCE_PATH = "compiler/"


def read_source_code_lines(dirpath: str) -> int:
    lines = 0
    if not isdir(dirpath):
        return -1

    for p in listdir(dirpath):
        if isdir(new_dir := join(dirpath, p)):
            lines += read_source_code_lines(new_dir)
        if p.endswith(".rs"):
            print(p)
            with open(join(dirpath, p), "r") as f:
                lines += len(f.readlines())

    return lines


def main():
    x = read_source_code_lines(SOURCE_PATH)
    if x == -1:
        print("source path is not a directory")
        return
    print(f"lines of elo: {x}")


if __name__ == "__main__":
    main()
