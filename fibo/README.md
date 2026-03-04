# RUST : Fibonacci Sequence Generator
A command-line tool that calculates and displays the Fibonacci sequence up to a user-specified limit.

## Features
- **User-Defined Limits**: Allows the user to input a maximum value (`n`) to control how far the sequence generates.
- **Dynamic Calculation**: Uses a `while` loop to update variables `a` and `b` iteratively, keeping memory usage constant.
- **Input Parsing**: Converts raw string input into an unsigned 32-bit integer (`u32`) using `.trim().parse()`.
- **Interactive CLI**: Prompts the user for input and displays the resulting sequence clearly on a single line.

## Usage
Run the program and enter a maximum value for the sequence:

```bash
cargo run
# When prompted, type a number (e.g., 100) and press Enter.
```

## What I Learned ?
- **User Documentation**: I learned that a good README should describe the "human-computer interaction," not just the code logic.
- **Edge Case Awareness**: I realized that if a user inputs something that isn't a number (like "abc"), the program will trigger the `.expect("Invalid number")` error I wrote in the code.
- **Instructional Clarity**: I learned that being specific about *when* to press Enter is helpful for beginner users.

## Scholarship Goals
*Empathy in Engineering*: My goal is to write documentation from the perspective of a user who has never seen my code before.*Technical Writing*: I am practicing how to explain technical "blocking" operations (like `read_line`) in simple, everyday language.
*Complete Documentation*: I aim to ensure my README files cover the full lifecycle of the program, from setup to final output.