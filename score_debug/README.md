# RUST : Simple CSV Data Parser
A Rust-based utility that reads student records from a CSV file, parses them into structured objects, and calculates statistical averages. This project demonstrates *File Handling*, *Error Propagation* and *Iterator Power* in Rust.

## Features
- **File Stream Processing** : Uses `BufReader` to read files line-by-line, which is memory-efficient for large datasets.
- **Robust CSV Parsing** : Implements a custom `from_csv_line` constructor that converts raw strings into a `Record` struct.
- **Dynamic Error Handling** : Utilizes `Box<dyn Error>` and the `?` operator to catch and report issues like missing files or incorrectly formatted numbers.
- **Data Analytics** : Uses high-level iterator methods `(map, sum)` to calculate the average score of all entries.

## The Architecture
1. **The Record Struct** : A data model representing a student with a `name`, `age`, and `score`.
2. **The Parser** : The `from_csv_line` function splits text by commas, validates the field count, and parses strings into numeric types.
3. **The Main Loop** : Opens `data.csv`, iterates through the lines, and pushes valid records into a `Vec<Record>`.

## Input File Format (data.csv)
The program expects a file named `data.csv` in the root directory:
```bash
Alice,14,100
Rahul,14,96
```

## What I Learned
- **Buffered Reading** : I learned that `io::BufReader` is better for reading files because it reduces the number of direct system calls to the hard drive.
- **Type Conversion** : I practiced using `.parse::<T>()` with the "turbofish" syntax to turn text into specific numbers.
- **Memory Management** : I understood that `.to_string()` creates a new owned string for the struct, while the initial split fields are just temporary references.
- **Dynamic Error Boxes** : I learned that `Box<dyn Error>` allows a function to return different types of errors (IO vs. Parsing) in a single unified way.
- **The Question Mark Operator (`?`)** : I practiced using `?` to stop a function early if an error occurs, making the code much cleaner than using match.
- **Iterator Math** : I learned how to use `.iter()` and `.map()` to extract specific fields from a list to perform calculations like `.sum()`.

## Scholarship Goals
- **Input Validation** : My next goal is to handle "bad inputs" (like empty lines) using a check like `line.is_empty()` to prevent the parser from failing.
- **Advanced Iterators** : I am practicing using `.enumerate()` so I can tell the user exactly which line number has an error if the parsing fails.
- **Vector Management** : I aim to learn how to sort the `Vec<Record>` by score to find the top-performing student.