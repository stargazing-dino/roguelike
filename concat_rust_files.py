import os

src_folder = "src"
output_file = "concatenated_rust_files.rs"


def main():
    """
    This script will create a new file named concatenated_rust_files.rs
    in the same directory as the script. The file will contain the
    content of all the .rs files in the src folder, with each file's
    content preceded by a comment containing the file name. The
    content of the files will be separated by two newlines.
    """
    with open(output_file, "w") as outfile:
        for root, _, files in os.walk(src_folder):
            for file in files:
                if file.endswith(".rs"):
                    file_path = os.path.join(root, file)
                    with open(file_path, "r") as infile:
                        outfile.write(f"// {file}\n\n")
                        outfile.write(infile.read())
                        outfile.write("\n\n")


if __name__ == "__main__":
    main()
