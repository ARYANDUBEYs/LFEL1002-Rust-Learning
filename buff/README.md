# RUST : Buffered File I/O & Error Propagation
This program demonstrates efficient file handling in Rust. It creates a file, writes data and then reads it back line-by-line using a buffer to optimize.

## Features
- **io::Result<()>**: Uses the idiomatic Rust return type for main to handle system errors.
- **Error Propagation (?)**: Replaces manual match blocks with the `?` operator for cleaner, more readable code.
- **BufReader**: Wraps the file in a buffer to read data efficiently from memory rather than the disk.
- **Line Iteration**: Uses `.lines()` to process the file content one string at a time.

## Usage
Run the program using cargo:
```bash
cargo run
```

## What I Learned?
- **The Unit Type ()**: Understood that `Ok(())` signals a successful operation when no specific return value is needed.
- **Trait Prelude**: Learned how `std::io::prelude::*` provides the traits necessary for line-based reading.
- **Buffered vs. Unbuffered**: Discovered that `BufReader` reduces the number of expensive system calls to the Operating System.
- **Module Aliasing**: Used use `std::io::{self, ...}` to cleanly access the io namespace for error handling.

## 🎓 Scholarship Goals (LiFT 2026)
By mastering buffered I/O and proper error propagation, I am building the foundational skills required to develop high-performance, reliable Linux system utilities in Rust.