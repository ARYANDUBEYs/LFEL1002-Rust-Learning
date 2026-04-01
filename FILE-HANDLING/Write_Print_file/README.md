# Rust File I/O: Write and Read
This is a very basic rust program which demonstrates how to interact with the file system. It creates a file, writes string into it and reads the content back to the console

## Features 
-**`std::fs` Module**: Utilises standard rust library for file system operations.
-**`fs::write`**: Creates `note.txt` and stores "Hi, Rust".
-**`fs::read_to_string`**: Retrieves data from the file `note.txt` as a string.

## Usage
Run the program using cargo:
```bash
cargo run
```
## What I learned? 
--**`std::fs` Library**: Learned basic file interactions like creating a file , writing in it and reading it.
--**Error handling**: Used .expect() to write a custom error message in case the program crashes.
--**Safety**: Learned how rust ensures memory safety during file operations by using the standard library's functions.

## Scholorship Goals
This project is stepping stone in my journey to master programming using Rust language. I am documenting these fundamentals as a part of my preparation for the **LiFT Scholorship** to demonstrate my dedication  commitment towards to building safe open-source software.