# RUST : Modular File I/O & Error Handling
This program demonstrates three ways to handle errors in Rust while performing file operations. It is split into modular functions to show how to pass file resources and manage results efficiently.

## Features
- **`match` Handling**: Used in `create_with_match` to provide custom error messages to the user.
- **The `?` Operator**: Used in `write_with_question_mark` for clean and automatic error propagation.
- **Manual Ok/Err**: Used in `read_with_buffer` to manually inspect and return results from a loop.
- **`BufReader`**: Implemented to read file data efficiently in chunks rather than one byte at a time.

## Usage
Run the program using cargo:
```bash
cargo run
```

## What I Learned?
- **Modular Design**: Learned how to break I/O logic into separate functions that return io::Result.
- **Manual vs. Auto**: Discovered that while `?` is faster to write, match and if let give more control over error reporting.
- **Result Construction**: Practiced using `Ok` and `Err` as "wrappers" to satisfy the function's return type.
- **Resource Ownership**: Successfully passed a `File` object from one function to another.

## Scholarship Goals (LiFT 2026)
This project proves I can write modular, organized Rust code. By mastering different error-handling patterns, I am preparing to build reliable tools for the Linux ecosystem.