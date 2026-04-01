# RUST : Security Scanner
This program scans a text file to find potential security leaks like passwords or API keys. It is designed to be fast, safe, and easy to use.This tool acts as an automated security auditor. It uses the Rust `regex` crate to search through local files for patterns that look like sensitive credentials. By reading line-by-line, it maintains a very small memory footprint, making it safe for use on systems with limited resources.

## Features
- **Infinite File Support**: Uses `BufReader` to process files one line at a time, preventing memory crashes on massive files.
- **Contextual Alerts**: Provides the exact line number and the matching text so developers can find and fix leaks instantly.
- **Case-Insensitive Search**: Employs the `(?i)` flag to ensure passwords are found regardless of capitalization.
- **Error Resilience**: Uses a `match` statement to skip corrupted lines and keep the scan running until the end of the file.

## How to Setup
1. Open your `Cargo.toml` file.
2. Add `regex = "1.12.3"` under the `[dependencies]` section.
3. Ensure you have a file named `input.txt` in your project root folder.

## Usage
To run the scanner, open your terminal in the project directory and type:
```bash
cargo run
```

## What I Learned
- **Iterator Pipelines**: I learned that for line in `reader.lines()` is a memory-efficient way to stream data instead of loading it all at once, preventing memory exhaustion.

- **Simultaneous Capture**: I learned how to use `.enumerate()` to link a line's physical location (the index) with its actual content at the same time.

- **Method Chaining Order**: I understood that `.lines().enumerate()` is the required order because you must define the data units (lines) before the program can count them.

- **Zero-based Indexing**: I learned to translate computer-style 0-counting into human-friendly 1-counting by adding + 1 to the output so it matches text editors.

- **Regex Grouping**: I learned that parentheses () define capture groups, which allows the program to isolate the secret value from the label (like "password").

## Scholarship Goals
*Mastering Rust Ownership*: My goal is to understand how Rust manages memory without a garbage collector by practicing with String and reference types like &str.

*Building Secure Software*: I am learning to identify common security patterns, such as leaked credentials, to build more robust and professional applications.

*Efficient Logic*: I aim to write code with "O(n)" complexity, ensuring the program only reads through the data once for maximum processing speed.