# RUST : Bike Journey Time Calculator
A Rust-based CLI tool that uses a custom `struct` to calculate the time required for a bike to cover a specific distance at a given speed.This program demonstrates how to model physical objects and their behaviors in Rust. It takes user input for a bike's name, speed, and distance, then uses a method to perform a division calculation ($Time = Distance / Speed$) to return the travel duration.

## Features
- **Custom Struct Modeling**: Defines a `Bike` with fields for `name`, `speed`, and `distance`.
- **Method Implementation**: Features a `time()` method within an `impl` block that accesses internal struct data to perform calculations.
- **Interactive Prompts**: Uses `print!` and `io::stdout().flush()` to ensure questions appear on the same line as the user's answer.
- **Type Conversion**: Converts user string input into `f64` integers using `.trim().parse()`.
- **Input Sanitization**: Cleans the bike name using `.trim().to_string()` to remove the invisible newline character.

## How to Setup
1. Ensure Rust is installed on your computer.
2. Create a new project: `cargo new bike_calculator`.
3. Replace the code in `src/main.rs` with the provided script.
4. Run using `cargo run`.

## ## Usage
Run the program and provide the requested information:
```bash
cargo run
# Enter the bike's name: Ninja
# Enter distance to cover: 100
# Enter speed: 20
#OUTPUT:
# My bike Ninja will take 5 units of time to reach the destination
```

## What I Learned
- **Variable Declaration** : I learned that every new variable, including Struct instances, must be initialized with the let keyword.
- **The Importance of Imports** : I discovered that the Write trait must be imported (`use std::io::Write`) for the `flush()` method to work.
- **Semicolon Precision** : I learned that assigning a Struct to a variable is an expression that requires a semicolon `;` at the end of the curly braces.
- **Method Return Types** : I practiced defining a method that returns a specific type (`-> f64`) for use in later logic.

## Scholarship Goals
- **Mathematical Edge Cases** : My goal is to learn how to handle "Division by Zero" errors if a user inputs a speed of `0`.
- **Precision Improvements** : I am practicing moving from integer math (`u64`) to floating-point math (`f64`) for more accurate time results with decimals.
- **Input Robustness** : I aim to replace `.unwrap()` with proper error handling to prevent the program from crashing on invalid inputs.