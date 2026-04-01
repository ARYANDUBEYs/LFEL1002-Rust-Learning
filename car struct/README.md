# Rust : Car Registry
A simple command-line tool that allows users to input details for multiple vehicles and stores them using a structured data model. This project demonstrates basic Rust concepts like *Structs*, *Input/Output (I/O) handling* and *Method implementation*.

## Features
- **Custom Struct Modeling** : Uses a Car struct to organize diverse data types like `String`, `u16`, and `u32`.
- **Dynamic Input Collection** : Leverages `std::io::{self, Write}` to capture real-time user data.
- **Smart Buffer Management** : Uses `input.clear()` efficiently to reuse a single String buffer for multiple inputs, saving memory.
- **Formatted Output** : Implements a `display()` method to print car details in a clean, readable list.
- **Safe Parsing** : Uses `.parse()` to convert string inputs into numbers (`year` and `mileage`) safely.

## Usage
1. Clone the project and navigate to the directory.
2. Run the application:
``` bash
cargo run
```
3. Follow the prompts: Enter the Make, Model, Year, Color, and Mileage for two different cars.

## What I Learned
- **Console Flushing** : I learned that `io::stdout().flush()` is necessary to ensure prompts appear before the program waits for input.
- **Method Syntax** : I practiced using `&self` in an `impl` block to allow a `struct` to access its own data without giving up ownership.
- **Input Cleaning** : I understood why `.trim()` is required to remove the "newline" character created when the user presses Enter.
- **Buffer Reuse** : I learned that clearing a string buffer with `.clear()` is more efficient than creating a new String for every single prompt.
- **Parsing** : I practiced converting `String` inputs into numeric types using `.trim().parse()`.
- **Ownership & Return** : I learned how to return a newly created struct instance from a function back to the `main` scope.

## Scholarship Goals
- **Input Validation** : My next goal is to handle "bad inputs" (like typing letters for the year) using match instead of `.expect()` so the program doesn't crash.
- **Code Dryness** : I aim to reduce repetition by creating a helper function for the `print!`, `flush()`, and `read_line()` pattern.