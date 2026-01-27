# simple joke script that counts the lines of the Pile's implementation code
from os.path import join, isdir
from os import listdir

SOURCE_PATH = "compiler/"


def read_source_code_info(dirpath: str) -> dict[str, int] | None:
    info = {
        "lines": 0,
        "semis": 0,
        "curly": 0,
    }
    if not isdir(dirpath):
        return None

    for p in listdir(dirpath):
        if isdir(new_dir := join(dirpath, p)):
            new_info = read_source_code_info(new_dir)
            if new_info is None:
                return None
            info = {
                "lines": info["lines"] + new_info["lines"],
                "semis": info["semis"] + new_info["semis"],
                "curly": info["curly"] + new_info["curly"],
            }
        if p.endswith(".rs"):
            print(p)
            with open(join(dirpath, p), "r") as f:
                for char in f.read():
                    info["curly"] += int(char in ('{', '}'))
                    info["semis"] += int(char == ';')
                    info["lines"] += int(char == '\n')
    return info


def main():
    info = read_source_code_info(SOURCE_PATH)
    if info is None:
        print("source path is not a directory")
        return
    print(f"lines of elo: {info['lines']}")
    print(f"curly braces of elo: {info['curly']}")
    print(f"semicolons of elo: {info['semis']}")


if __name__ == "__main__":
    main()
