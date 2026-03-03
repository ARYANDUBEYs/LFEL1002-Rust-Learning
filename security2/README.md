# RUST : Security Scanner in Rust (Interactive Version)
This program allows a user to type a file path and then scans that file for potential security leaks like passwords or API keys. It is designed for safety, memory efficiency, and a smooth user experience.This tool acts as an automated security auditor. It uses the Rust `regex` crate to search through local files for patterns that look like sensitive credentials. By using `BufReader`, it processes files line-by-line, ensuring it can handle massive files without crashing your computer's RAM.

## Features
- **Dynamic File Selection**: Instead of a hard-coded file name, the user can type any file path at runtime.
- **Responsive Prompting**: Uses `print!` and manual `flush()` to make the input request appear instantly on the same line.
- **Clean Path Handling**: Employs `.trim()` to remove invisible "Enter" characters, ensuring the file path is readable by the operating system.
- **Contextual Alerts**: Provides the exact line number and the matching text for every security leak found.
- **Error Resilience**: Uses `match` statements to skip unreadable lines and handles "File Not Found" errors gracefully.

## How to Setup
1. Open your `Cargo.toml` file.
2. Add `regex = "1.10"` under the `[dependencies]` section.
3. Ensure you have a file (like `secrets.txt`) ready to scan.

## Usage
Run the program using Cargo:
```bash
cargo run
```

## What I Learned ?
- **Interactive Stdin**: I learned how to use `io::stdin().read_line()` to make my program interactive instead of static.

- **The Necessity of Flush**: I discovered that print! stays in the buffer "waiting room," so I must use `io::stdout().flush()` to show the prompt immediately.

- **Sanitizing Inputs**: I learned that `.trim()` is required to snip off the hidden `\n` from the user's input so the file path is valid.

- **Iterator Pipelines**: I practiced using `.lines().enumerate()` to efficiently track line numbers while streaming file content.

- **Regex Groups**: I learned how to use capture groups () to isolate specific secret values within a line of text.

## Scholarship Goals
*Building User-Friendly CLI*: My goal is to master the small details (like flush and trim) that make a command-line tool feel professional and easy to use.

*Efficient Data Processing*: I am practicing "O(n)" logic to ensure my security scanner remains fast regardless of file size.

*Mastering Ownership*: I am learning how Rust handles String ownership when passing user input to file-opening functions.