# RUST : Input and Output
A practical Rust program that demonstrates how to create a file, write data to it, read that data back, perform calculations, and save the results into a new output file.

## Features
- **Automated File Creation**: Uses `File::create` to generate both input and output files directly from the code.
- **Buffered Reading**: Utilizes `BufReader` to read the file line-by-line, which is much more memory-efficient than loading a whole file at once.
- **Data Parsing**: Employs `.trim().parse()` to convert raw text from a file into a usable integer (`i32`) for math operations.
- **Double-Stream I/O**: Demonstrates writing to the terminal with `println!` and writing to a file with `writeln!` simultaneously.
- **Error Propagation**: Uses the `?` operator and `io::Result<()>` to handle potential file errors safely and cleanly.

## Usage
Run the program in your terminal:
```bash
cargo run
```

## What I Learned
- **The `?` Operator**: I learned that `?` is a shorthand for returning an error early if a file operation fails, keeping the code clean.
- **String to Integer**: I practiced using `.parse()` to turn text data into numbers so I can perform calculations.
- **Writing vs. Writeln**: I learned that `write!` puts text in a file, while `writeln!` adds a newline at the end, which is important for formatting file data.
- **Return Types**: I learned that `fn main() -> io::Result<()>` allows the main function itself to report if something went wrong during file access.
- **Line Iteration**: I understood that `reader.lines()` creates an iterator that lets me process a file one piece at a time.

## Scholarship Goals
- *Mastering the Standard Library*: My goal is to become proficient with the std::fs and std::io modules to build tools that interact with the computer's file system.
- *Building Data Pipelines*: I am learning how to move data between different formats and files, a core skill for backend and data engineering.
- *Type Safety*: I aim to master Rust's strict typing system, ensuring that data parsed from files is always handled correctly to avoid program crashes.