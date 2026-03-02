# RUST : Regex Pattern Matching in Rust
This program demonstrates how to use the regex crate to search for specific data patterns within strings. It moves beyond simple word-searching to find types of data, such as numeric codes, using regular expressions.

## Features
- **Regex Compilation**: Uses `Regex::new` to build a searchable pattern from a raw string.
- **Pattern Validation**: Emplements `is_match` to quickly verify if a pattern exists within a text block.
- **Iterative Extraction**: Uses `.find_iter` to loop through a string and pull out every individual match found.
- **Raw String Syntax**: Utilizes `r"..."` to write clean regex patterns without needing complex escape characters.

## What I Learned?
- **External Crates**: Learned that specialized tools like `regex` must be added to `Cargo.toml` as they are not in the standard library.
- **Safe Compilation**: Discovered that `Regex::new` returns a Result, requiring handling in case the regex pattern is mathematically invalid.
- **Pattern Logic**: Practiced using `\d` tokens to represent digits, allowing the program to find numbers regardless of their specific value.

## Scholarship Goals (LiFT 2026)
Mastering regex is a vital step for Linux systems programming. This skill allows me to write tools that can parse logs, audit configurations, and automate security scans within a Linux environment.