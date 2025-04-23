import os

def count_rust_lines(directory):
    total_lines = 0
    rust_files = 0
    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                print(f"{file_path=}")
                try:
                    with open(file_path, 'r', encoding='utf-8') as f:
                        lines = f.readlines()
                        line_count = len([line for line in lines if line.strip()])  # Ignore blank lines
                        total_lines += line_count
                        rust_files += 1
                except (UnicodeDecodeError, FileNotFoundError) as e:
                    print(f"Skipping file {file_path}: {e}")

    print(f"Rust files found: {rust_files}")
    print(f"Total non-blank lines of Rust code: {total_lines}")

# Example usage
if __name__ == "__main__":
    import sys
    if len(sys.argv) > 1:
        directory = sys.argv[1]
    else:
        directory = os.getcwd()  # Use current directory if none specified
    count_rust_lines(directory)